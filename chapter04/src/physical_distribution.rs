#![allow(dead_code)]

#[derive(Debug)]
struct PhysicalDistributionBase {}

impl PhysicalDistributionBase {
    fn new() {
        unimplemented!()
    }

    // 出庫
    fn ship(&self, _baggage: &Baggage) -> Baggage {
        unimplemented!()
    }

    // 入庫
    fn receive(&self, _baggage: &Baggage) {
        unimplemented!()
    }
}

#[derive(Debug)]
struct Baggage {}

// 輸送用の Domain Service
#[derive(Debug)]
struct TransportService {}

impl TransportService {
    fn transport(from: PhysicalDistributionBase, to: PhysicalDistributionBase, _baggage: Baggage) {
        let _shipped_baggage = from.ship(&_baggage);
        to.receive(&_baggage);

        // 配送の記録を行う
    }
}
