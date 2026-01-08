// OSC Command Handlers
(
OSCdef(\monokit_trigger, { |msg|
    ~noise.set(\t_gate, 1);
    ~mod.set(\t_gate, 1);
    ~primary.set(\t_gate, 1);
    ~main.set(\t_gate, 1);
    SystemClock.sched(0.01, {
        ~noise.set(\t_gate, 0);
        ~mod.set(\t_gate, 0);
        ~primary.set(\t_gate, 0);
        ~main.set(\t_gate, 0);
        nil;
    });
}, '/monokit/trigger');

OSCdef(\monokit_volume, { |msg|
    var vol = msg[1].asFloat.clip(0, 1);
    ~main.set(\volume, vol);
}, '/monokit/volume');

OSCdef(\monokit_param, { |msg|
    var paramName = msg[1].asSymbol;
    var paramValue = msg[2].asFloat;
    var routeParam = { |pName, pValue|
        case
            { pName == \scopeGain } { ~scope.set(pName, pValue) }
            { #[nw, nv].includes(pName) } { ~noise.set(pName, pValue) }
            { #[mf, mw, mv, fb, fba, fbd, nm, mb, mba, mbd, md, fbev_atk, fbev_crv].includes(pName) } { ~mod.set(pName, pValue) }
            { #[pf, pw, pv, fm, fa, fd, pa, pd, np, tk, mt, mp, penv_atk, penv_crv, fmev_atk, fmev_crv].includes(pName) } { ~primary.set(pName, pValue) }
            { pName == \t_gate_plaits } { ~plaits.set(\t_gate, pValue) }
            { #[engine, harmonics, timbre, morph, decay, lpg, plv, pav, pitch, detune].includes(pName) } { ~plaits.set(pName, pValue) }
            { #[env_atk, env_crv].includes(pName) } {
                ~mod.set(pName, pValue);
                ~primary.set(pName, pValue);
                ~main.set(pName, pValue);
            }
            { true } { ~main.set(pName, pValue) };
    };
    routeParam.(paramName, paramValue);
}, '/monokit/param');

OSCdef(\monokit_scope_rate, { |msg|
    var timeMs = msg[1].asFloat.clip(5, 500);
    var rate = 128 / (timeMs * 44.1);
    ~scope.set(\scopeRate, rate);
}, '/monokit/scope/rate');

~recPath = nil;
~recIndex = 0;

OSCdef(\monokit_rec, { |msg|
    var path, dir;
    if(s.isRecording.not, {
        dir = if(msg[1].notNil, { msg[1].asString }, { "." });

        if(~recPath.notNil, {
            path = ~recPath ++ "_" ++ ~recIndex ++ ".wav";
            ~recIndex = ~recIndex + 1;
        }, {
            path = dir +/+ "monokit_" ++ Date.getDate.stamp ++ ".wav";
        });

        s.recHeaderFormat = "wav";
        s.recSampleFormat = "int24";
        s.record(path);
        ("REC: " ++ path).postln;
    }, {
        "Already recording".postln;
    });
}, '/monokit/rec');

OSCdef(\monokit_rec_stop, { |msg|
    if(s.isRecording, {
        s.stopRecording;
        "Recording stopped".postln;
    }, {
        "Not recording".postln;
    });
}, '/monokit/rec/stop');

OSCdef(\monokit_rec_path, { |msg|
    var pathPrefix = msg[1].asString;
    ~recPath = pathPrefix;
    ~recIndex = 0;
    ("Recording path prefix: " ++ pathPrefix).postln;
}, '/monokit/rec/path');

OSCdef(\monokit_slew, { |msg|
    var slewMs = msg[1].asFloat.clip(0, 10000);
    var slewSec = slewMs / 1000;
    ~noise.set(\slew_time, slewSec);
    ~mod.set(\slew_time, slewSec);
    ~primary.set(\slew_time, slewSec);
    ~main.set(\slew_time, slewSec);
    ("SLEW TIME SET TO" + slewMs + "MS").postln;
}, '/monokit/slew');

OSCdef(\monokit_slew_param, { |msg|
    var paramName = msg[1].asString;
    var slewTime = msg[2].asFloat.clip(0, 10);
    var slewArgName = ("slew_" ++ paramName).asSymbol;
    var paramSym = paramName.asSymbol;

    case
        { #[mf, fb].includes(paramSym) } { ~mod.set(slewArgName, slewTime) }
        { #[pf, fm].includes(paramSym) } { ~primary.set(slewArgName, slewTime) }
        { true } { ~main.set(slewArgName, slewTime) };

    ("SLEW" + paramName.toUpper + "SET TO" + (slewTime * 1000) + "MS").postln;
}, '/monokit/slew/param');

~diagEnabled = false;
~diagTimestamps = [];
~diagMaxSamples = 1000;
~triggerCount = 0;

OSCdef(\monokit_trigger).free;
OSCdef(\monokit_trigger, { |msg|
    if(~diagEnabled, {
        ~triggerCount = ~triggerCount + 1;
        ~diagTimestamps = ~diagTimestamps.add(Main.elapsedTime);
        if(~diagTimestamps.size > ~diagMaxSamples, {
            ~diagTimestamps = ~diagTimestamps.drop(1);
        });
    });
    ~noise.set(\t_gate, 1);
    ~mod.set(\t_gate, 1);
    ~primary.set(\t_gate, 1);
    ~main.set(\t_gate, 1);
    SystemClock.sched(0.01, {
        ~noise.set(\t_gate, 0);
        ~mod.set(\t_gate, 0);
        ~primary.set(\t_gate, 0);
        ~main.set(\t_gate, 0);
        nil;
    });
}, '/monokit/trigger');

OSCdef(\monokit_diag, { |msg|
    var enable = msg[1].asInteger;
    ~diagEnabled = enable > 0;
    if(~diagEnabled, {
        ~diagTimestamps = [];
        ~triggerCount = 0;
        "SC TIMING DIAGNOSTICS ENABLED".postln;
    }, {
        "SC TIMING DIAGNOSTICS DISABLED".postln;
    });
}, '/monokit/diag');

OSCdef(\monokit_diag_report, { |msg|
    var intervals, oddIntervals, evenIntervals, avgOdd, avgEven, swingRatio;
    var file, reportPath, minInterval, maxInterval, avgInterval, stdDev;
    var sum, sumSq, variance;

    ("SC DIAG: " ++ ~triggerCount ++ " triggers received, " ++ ~diagTimestamps.size ++ " timestamps recorded").postln;

    if(~diagTimestamps.size < 2, {
        "SC DIAG: Insufficient data (need at least 2 triggers)".postln;
    }, {
        intervals = (~diagTimestamps.size - 1).collect({ |i|
            (~diagTimestamps[i + 1] - ~diagTimestamps[i]) * 1000;
        });

        oddIntervals = [];
        evenIntervals = [];
        intervals.do({ |val, idx|
            if(idx.odd, {
                oddIntervals = oddIntervals.add(val);
            }, {
                evenIntervals = evenIntervals.add(val);
            });
        });

        avgInterval = intervals.mean;
        minInterval = intervals.minItem;
        maxInterval = intervals.maxItem;

        sum = intervals.sum;
        sumSq = intervals.collect({ |x| x * x }).sum;
        variance = (sumSq / intervals.size) - (avgInterval * avgInterval);
        stdDev = variance.sqrt;

        avgOdd = if(oddIntervals.size > 0, { oddIntervals.mean }, { 0 });
        avgEven = if(evenIntervals.size > 0, { evenIntervals.mean }, { 0 });
        swingRatio = if(avgEven > 0, { avgOdd / avgEven }, { 1 });

        reportPath = "/Users/why/repos/monokit/sc_timing_report.txt";
        file = File.open(reportPath, "w");

        file.write("=== SuperCollider Timing Report ===\n");
        file.write("Total triggers received: " ++ ~triggerCount ++ "\n");
        file.write("Total timestamps recorded: " ++ ~diagTimestamps.size ++ "\n");
        file.write("Total intervals: " ++ intervals.size ++ "\n\n");
        file.write("Interval Statistics (ms):\n");
        file.write("  Mean: " ++ avgInterval.round(0.001) ++ "\n");
        file.write("  Min: " ++ minInterval.round(0.001) ++ "\n");
        file.write("  Max: " ++ maxInterval.round(0.001) ++ "\n");
        file.write("  Std Dev: " ++ stdDev.round(0.001) ++ "\n");
        file.write("  Jitter: " ++ (maxInterval - minInterval).round(0.001) ++ "\n\n");
        file.write("Swing Analysis:\n");
        file.write("  Even intervals avg: " ++ avgEven.round(0.001) ++ " ms\n");
        file.write("  Odd intervals avg: " ++ avgOdd.round(0.001) ++ " ms\n");
        file.write("  Ratio (odd/even): " ++ swingRatio.round(0.0001) ++ "\n");
        if((swingRatio - 1.0).abs > 0.02, {
            file.write("  >>> SWING DETECTED AT SC LEVEL! <<<\n");
        });
        file.write("\nFirst 16 intervals (ms):\n");
        intervals.keep(16).do({ |val, idx|
            file.write("  " ++ val.round(0.01) ++ "\n");
        });
        file.write("==========================================\n");

        file.close;

        "SC DIAG: Report written to sc_timing_report.txt".postln;
        ("SC DIAG: Mean=" ++ avgInterval.round(0.01) ++ "ms, Swing ratio=" ++ swingRatio.round(0.0001)).postln;
    });
}, '/monokit/diag/report');

OSCdef(\monokit_audio_out_list, { |msg|
    var devices = ServerOptions.outDevices;
    var current = s.options.outDevice ? "default";
    ("MONOKIT: Sending device list, current=" ++ current).postln;
    devices.do({ |d| ("  - " ++ d).postln });
    ~meterAddr.sendMsg('/monokit/audio/out/list', current, *devices);
}, '/monokit/audio/out/query');

"monokit server ready".postln;
)
