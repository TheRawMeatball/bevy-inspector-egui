use crate::options::{NumberAttributes, OptionAttributes, Vec2dAttributes};
use crate::{Context, Inspectable};
use bevy::asset::HandleId;
use bevy::pbr2::StandardMaterial;
use bevy::prelude::*;
use bevy::render2::render_resource::Texture;
use bevy_egui::egui;
use egui::Grid;

impl_for_struct_delegate_fields!(
    bevy::pbr2::PointLight:
    color,
    intensity with NumberAttributes::positive().speed(1.0),
    range with NumberAttributes::positive(),
    radius with NumberAttributes::positive(),
);

//////// COMPONENTS ////////

impl Inspectable for Transform {
    type Attributes = ();

    fn ui(
        &mut self,
        ui: &mut bevy_egui::egui::Ui,
        _options: Self::Attributes,
        context: &Context,
    ) -> bool {
        let mut changed = false;
        ui.vertical_centered(|ui| {
            Grid::new(context.id()).show(ui, |ui| {
                ui.label("Translation");
                changed |= self.translation.ui(ui, Default::default(), context);
                ui.end_row();

                ui.label("Rotation");
                changed |= self.rotation.ui(ui, Default::default(), context);
                self.rotation = self.rotation.normalize();
                ui.end_row();

                ui.label("Scale");
                let scale_attributes = NumberAttributes {
                    min: Some(Vec3::splat(0.0)),
                    ..Default::default()
                };
                changed |= self.scale.ui(ui, scale_attributes, context);
                ui.end_row();
            });
        });
        changed
    }
}

impl Inspectable for GlobalTransform {
    type Attributes = <Transform as Inspectable>::Attributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) -> bool {
        let global_transform = std::mem::take(self);

        let mut transform = Transform {
            translation: global_transform.translation,
            rotation: global_transform.rotation,
            scale: global_transform.scale,
        };

        let changed = transform.ui(ui, options, context);

        *self = GlobalTransform {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: transform.scale,
        };

        changed
    }
}

impl Inspectable for Mat3 {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, context: &Context) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            changed |= self.x_axis.ui(ui, Default::default(), context);
            changed |= self.y_axis.ui(ui, Default::default(), context);
            changed |= self.z_axis.ui(ui, Default::default(), context);
        });
        changed
    }
}

impl Inspectable for Mat4 {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, context: &Context) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            changed |= self.x_axis.ui(ui, Default::default(), context);
            changed |= self.y_axis.ui(ui, Default::default(), context);
            changed |= self.z_axis.ui(ui, Default::default(), context);
            changed |= self.w_axis.ui(ui, Default::default(), context);
        });
        changed
    }
}

#[derive(Default, Debug, Clone)]
pub struct ColorAttributes {
    pub alpha: bool,
}

impl Inspectable for bevy::render2::color::Color {
    type Attributes = ColorAttributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, _: &Context) -> bool {
        let old: [f32; 4] = (*self).into();

        if options.alpha {
            let mut color = egui::color::Color32::from_rgba_premultiplied(
                (old[0] * u8::MAX as f32) as u8,
                (old[1] * u8::MAX as f32) as u8,
                (old[2] * u8::MAX as f32) as u8,
                (old[3] * u8::MAX as f32) as u8,
            );
            let changed = ui.color_edit_button_srgba(&mut color).changed();
            let [r, g, b, a] = color.to_array();
            *self = bevy::render2::color::Color::rgba_u8(r, g, b, a);

            changed
        } else {
            let mut color = [old[0], old[1], old[2]];
            let changed = ui.color_edit_button_rgb(&mut color).changed();
            let [r, g, b] = color;
            *self = bevy::render2::color::Color::rgba(r, g, b, old[3]);

            changed
        }
    }
}

////// OTHER //////

#[rustfmt::skip]
impl Inspectable for bevy::pbr2::StandardMaterial {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, context: &Context) -> bool {
        let mut changed = false;
        ui.vertical_centered(|ui| {
            egui::Grid::new(context.id()).show(ui, |ui| {
                ui.columns(2, |all| {
                    egui::Grid::new("left").show(&mut all[0], |ui| {
                        ui.label("base_color");
                        changed |= self.base_color.ui(ui, Default::default(), context);
                        ui.end_row();

                        ui.label("perceptual_roughness");
                        changed |= self.perceptual_roughness.ui(ui, NumberAttributes::between(0.089, 1.0).speed(0.01), context);
                        ui.end_row();

                        ui.label("reflectance");
                        changed |= self.reflectance.ui(ui, NumberAttributes::positive(), context);
                        ui.end_row();
                    });
                    egui::Grid::new("right").show(&mut all[1], |ui| {
                        ui.label("emissive");
                        changed |= self.emissive.ui(ui, Default::default(), context);
                        ui.end_row();

                        ui.label("metallic");
                        changed |= self.metallic.ui(ui, NumberAttributes::normalized().speed(0.01), context);
                        ui.end_row();

                        ui.label("unlit");
                        changed |= self.unlit.ui(ui, Default::default(), context);
                        ui.end_row();
                    });
                });
            });

            ui.collapsing("Textures", |ui| {
                egui::Grid::new("Textures").show(ui, |ui| {
                    let texture_option_attributes = OptionAttributes { replacement: Some(|| Handle::weak(HandleId::random::<StandardMaterial>())), ..Default::default() };

                    ui.label("base_color");
                    changed |= self.base_color_texture.ui(ui, texture_option_attributes.clone(), &context.with_id(0));
                    ui.end_row();

                    // ui.label("normal_map");
                    // changed |= self.normal_map.ui(ui, texture_option_attributes.clone(), &context.with_id(0));
                    // ui.end_row();

                    ui.label("metallic_roughness");
                    changed |= self.metallic_roughness_texture.ui(ui, texture_option_attributes.clone(), &context.with_id(1));
                    ui.end_row();

                    ui.label("emmissive");
                    changed |= self.emissive_texture.ui(ui, texture_option_attributes.clone(), &context.with_id(2));
                    ui.end_row();

                    ui.label("occlusion texture");
                    changed |= self.occlusion_texture.ui(ui, texture_option_attributes, &context.with_id(3));
                    ui.end_row();
                });
            });
        });
        changed
    }
}

impl Inspectable for Name {
    type Attributes = ();

    fn ui(&mut self, ui: &mut egui::Ui, _: Self::Attributes, _: &Context) -> bool {
        ui.label(self.as_str());
        false
    }
}

impl<'a, T: Inspectable> Inspectable for Mut<'a, T> {
    type Attributes = T::Attributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) -> bool {
        (**self).ui(ui, options, context)
    }
}
