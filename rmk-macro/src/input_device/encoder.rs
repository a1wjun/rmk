use quote::{format_ident, quote};
use rmk_config::{ChipModel, EncoderConfig};

use super::Initializer;
use crate::gpio_config::convert_gpio_str_to_input_pin;

/// Expand encoder device, this function returns the (device_initializer, processor_initializer)
///
/// `id_offset` is the offset of the encoder id, it is used to distinguish the encoder id between central and peripheral
pub(crate) fn expand_encoder_device(
    id_offset: usize,
    encoder_config: Vec<EncoderConfig>,
    chip: &ChipModel,
) -> (Vec<Initializer>, Vec<Initializer>) {
    if encoder_config.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut processor_initializer = vec![];
    let mut device_initializer = vec![];

    // Add encoder processor
    let encoder_processor_ident = format_ident!("encoder_processor");
    processor_initializer.push(Initializer {
        initializer: quote! {
            let mut #encoder_processor_ident = ::rmk::input_device::rotary_encoder::RotaryEncoderProcessor::new(&keymap);
        },
        var_name: encoder_processor_ident,
    });

    // Create rotary encoders
    for (idx, encoder) in encoder_config.iter().enumerate() {
        let encoder_id = idx as u8 + id_offset as u8;

        let pull = if encoder.internal_pullup { Some(true) } else { None };

        // Initialize pins
        let pin_a = convert_gpio_str_to_input_pin(chip, encoder.pin_a.clone(), false, pull);
        let pin_b = convert_gpio_str_to_input_pin(chip, encoder.pin_b.clone(), false, pull);

        let encoder_name = format_ident!("encoder_{}", encoder_id);
        // encoder_names.push(encoder_name.clone());

        // Create different types of encoders based on the phase field
        let encoder_device = match encoder.phase.as_deref() {
            Some("e8h7") => {
                quote! {
                    let mut #encoder_name = ::rmk::input_device::rotary_encoder::RotaryEncoder::with_phase(
                        #pin_a,
                        #pin_b,
                        ::rmk::input_device::rotary_encoder::E8H7Phase,
                        #encoder_id
                    );
                }
            }
            Some("resolution") => {
                // When phase is "resolution", ensure resolution and reverse are set
                let resolution = encoder
                    .resolution
                    .expect("Resolution value must be specified when phase is 'resolution'");
                let reverse = encoder.reverse.unwrap_or(false);

                quote! {
                    let mut #encoder_name = ::rmk::input_device::rotary_encoder::RotaryEncoder::with_resolution(
                        #pin_a,
                        #pin_b,
                        #resolution,
                        #reverse,
                        #encoder_id
                    );
                }
            }
            Some("default") => {
                // Default phase
                quote! {
                    let mut #encoder_name = ::rmk::input_device::rotary_encoder::RotaryEncoder::with_phase(
                        #pin_a,
                        #pin_b,
                        ::rmk::input_device::rotary_encoder::DefaultPhase,
                        #encoder_id
                    );
                }
            }
            _ => {
                panic!("Invalid rotary encoder phase, available phase: default, resolution, e8h7");
            }
        };

        device_initializer.push(Initializer {
            initializer: encoder_device,
            var_name: encoder_name,
        });
    }

    (device_initializer, processor_initializer)
}
