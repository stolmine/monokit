// OSC Forwarding - Meter, Spectrum, Scope, Compressor, CPU
(
~meterAddr = NetAddr("127.0.0.1", 57121);

~meterAddr.sendMsg('/monokit/ready');
"MONOKIT_SC_READY".postln;

OSCdef(\monokit_meter_fwd, { |msg|
    var channel = msg[2];
    var peak = msg[3];
    var rms = msg[4];
    ~meterAddr.sendMsg('/monokit/meter', channel, peak, rms);
}, '/monokit/meter');

~spectrumAddr = NetAddr("127.0.0.1", 57121);
OSCdef(\monokit_spectrum_fwd, { |msg|
    var data = msg.copyRange(3, 32);
    ~spectrumAddr.sendMsg('/monokit/spectrum', *data);
}, '/monokit/spectrum');

OSCdef(\monokit_scope_fwd, { |msg|
    var samples = msg.copyRange(3, 130);
    ~meterAddr.sendMsg('/monokit/scope', *samples);
}, '/monokit/scope');

OSCdef(\monokit_comp_fwd, { |msg|
    var input = msg[3];
    var output = msg[4];
    var gr_db = msg[5];
    ~meterAddr.sendMsg('/monokit/comp', input, output, gr_db);
}, '/monokit/comp');

~cpuRoutine = Routine({
    inf.do {
        ~meterAddr.sendMsg('/monokit/cpu', s.avgCPU, s.peakCPU);
        0.5.wait;
    };
}).play;
)
