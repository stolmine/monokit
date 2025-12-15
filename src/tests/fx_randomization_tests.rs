use super::common::create_test_metro_tx;
use crate::commands::randomization::{handle_rnd_fx, handle_rnd_filt, handle_rnd_dly, handle_rnd_verb};

#[test]
fn test_rnd_fx_executes() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_fx(&metro_tx, 5, |msg| {
        output_called = true;
        assert!(msg.contains("RANDOMIZED FX"));
    });
    assert!(result.is_ok());
    assert!(output_called);
}

#[test]
fn test_rnd_filt_executes() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_filt(&metro_tx, 5, |msg| {
        output_called = true;
        assert!(msg.contains("RANDOMIZED FILTER"));
    });
    assert!(result.is_ok());
    assert!(output_called);
}

#[test]
fn test_rnd_dly_executes() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_dly(&metro_tx, 5, |msg| {
        output_called = true;
        assert!(msg.contains("RANDOMIZED DELAY"));
    });
    assert!(result.is_ok());
    assert!(output_called);
}

#[test]
fn test_rnd_verb_executes() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_verb(&metro_tx, 5, |msg| {
        output_called = true;
        assert!(msg.contains("RANDOMIZED REVERB"));
    });
    assert!(result.is_ok());
    assert!(output_called);
}

#[test]
fn test_rnd_fx_sends_all_params() {
    let (metro_tx, rx) = create_test_metro_tx();
    let result = handle_rnd_fx(&metro_tx, 0, |_| {});
    assert!(result.is_ok());

    let mut param_count = 0;
    while let Ok(_) = rx.try_recv() {
        param_count += 1;
    }
    assert_eq!(param_count, 38);  // filter(4) + lofi(3) + ringmod(3) + reso(3) + delay(4) + eq(3) + reverb(4) + comp(1) + clouds(13)
}

#[test]
fn test_rnd_filt_sends_filter_params() {
    let (metro_tx, rx) = create_test_metro_tx();
    let result = handle_rnd_filt(&metro_tx, 0, |_| {});
    assert!(result.is_ok());

    let mut param_count = 0;
    while let Ok(_) = rx.try_recv() {
        param_count += 1;
    }
    assert_eq!(param_count, 4);
}

#[test]
fn test_rnd_dly_sends_delay_params() {
    let (metro_tx, rx) = create_test_metro_tx();
    let result = handle_rnd_dly(&metro_tx, 0, |_| {});
    assert!(result.is_ok());

    let mut param_count = 0;
    while let Ok(_) = rx.try_recv() {
        param_count += 1;
    }
    assert_eq!(param_count, 4);
}

#[test]
fn test_rnd_verb_sends_reverb_params() {
    let (metro_tx, rx) = create_test_metro_tx();
    let result = handle_rnd_verb(&metro_tx, 0, |_| {});
    assert!(result.is_ok());

    let mut param_count = 0;
    while let Ok(_) = rx.try_recv() {
        param_count += 1;
    }
    assert_eq!(param_count, 4);
}

#[test]
fn test_rnd_fx_no_output_when_debug_low() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_fx(&metro_tx, 0, |_| {
        output_called = true;
    });
    assert!(result.is_ok());
    assert!(!output_called);
}

#[test]
fn test_rnd_fx_output_when_debug_high() {
    let (metro_tx, _rx) = create_test_metro_tx();
    let mut output_called = false;
    let result = handle_rnd_fx(&metro_tx, 5, |_| {
        output_called = true;
    });
    assert!(result.is_ok());
    assert!(output_called);
}
