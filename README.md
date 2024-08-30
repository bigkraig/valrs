A tool to compare PIWIS VAL zips.

```
$ cargo run diff data/FAP_WP0XXXXXXXX_20240823_154128_23.0.1.zip data/FAP_WP0XXXXXXX_20240823_152849_23.0.1.zip
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/valrs diff data/FAP_WP0BC2Y15NSA74084_20240823_154128_23.0.1.zip data/FAP_WP0BC2Y15NSA74084_20240823_152849_23.0.1.zip`
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) central_locking_system_side_selective_at_mmi // not_active -> active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) IDG_rf_power // high_rf_power -> low_rf_power
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) trunk_entrapment_led // not_installed -> installed
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) emergency_brake_indication // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) emergency_brake_indication_trailer_plugged_in // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) recasflashing_via_psp // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) restricted_enabling_trunk_lid_push_button // not_active -> active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) comfort_operation // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) comfort_operation_radio_remote_control // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) comfort_operation_kessy // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) front_trunk_entrapment // not_active -> active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) automatic_unlock_nar // not_active -> active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) warning_tone_open_alarm_contact // active -> not_active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Kodierwert: Bitfield (2) horn_acknowledgment // not_active -> active
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_key_1: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_key_2: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_key_3: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_key_4: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_1: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_2: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_3: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_4: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_5: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_6: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_user_account_7: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // Personalized_settings_vehicle: Bitfield Acoustic_acknowlagement_signal // off -> on
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // pwm_levels_hl_14: pwm_low // 255 -> 200
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // pwm_levels_hl_14: pwm_high_1 // 200 -> 255
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // pwm_levels_hl_4: pwm_low // 255 -> 200
BCM2_MLBevo_HellaConti_PO_020 // Control unit, coding // pwm_levels_hl_4: pwm_high_1 // 200 -> 255
Section Ignition Key not found in other result
```
