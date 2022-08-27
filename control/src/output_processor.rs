use proton_peripherals::cv_output::CvOutput;
use proton_peripherals::gate_output::GateOutputExt;

use crate::output_request::OutputRequest;

pub struct OutputProcessor<G1, G2, G3, CO1, CO2> {
    gate_1: G1,
    gate_2: G2,
    gate_3: G3,
    cv_output_1: CO1,
    cv_output_2: CO2,
}

impl<G1, G2, G3, CO1, CO2> OutputProcessor<G1, G2, G3, CO1, CO2>
where
    G1: GateOutputExt,
    G2: GateOutputExt,
    G3: GateOutputExt,
    CO1: CvOutput,
    CO2: CvOutput,
{
    pub fn new(gate_1: G1, gate_2: G2, gate_3: G3, cv_output_1: CO1, cv_output_2: CO2) -> Self {
        Self {
            gate_1,
            gate_2,
            gate_3,
            cv_output_1,
            cv_output_2,
        }
    }

    pub fn apply(&mut self, request: OutputRequest) {
        self.gate_1.set_value(request.gate[0].value);
        self.gate_2.set_value(request.gate[1].value);
        self.gate_3.set_value(request.gate[2].value);
        self.cv_output_1.set_value(request.cv[0].value);
        self.cv_output_2.set_value(request.cv[1].value);
    }
}
