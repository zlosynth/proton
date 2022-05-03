use crate::model::state::State;

pub trait Module<N, NI, CI, PI>
where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ProducerIndex = PI, ConsumerIndex = CI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
{
    fn register(
        &self,
        graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
        state: &mut State<NI, CI, PI>,
    );
}
