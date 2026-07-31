use bevy::{
    ecs::{schedule::ScheduleConfigs, system::ScheduleSystem},
    prelude::*,
};

use crate::{
    Consumer, Producer,
    components::Offer,
    resources::{WorldState, update},
};

pub fn get_simulation_step() -> ScheduleConfigs<ScheduleSystem> {
    (
        pay_consumers,
        update_consumer_desires,
        producer_setup_shop,
        consumers_consumer,
        producers_make_orders,
        // update world
        update,
        // debug stuff
        debug_log,
    )
        .chain()
}

pub fn setup_consumers(mut commands: Commands) {
    let consumer = Consumer::new_random();
    commands.spawn(consumer);
}

pub fn setup_producers(mut commands: Commands) {
    let producer = Producer::new_random();
    commands.spawn(producer);
}

pub fn pay_consumers(mut query: Query<&mut Consumer>, day: Res<WorldState>) {
    for mut consumer in &mut query {
        consumer.as_mut().pay_if_pay_day(day.today());
    }
}

pub fn update_consumer_desires(mut query: Query<&mut Consumer>) {
    for mut consumer in &mut query {
        consumer.as_mut().update_desire();
    }
}

pub fn producer_setup_shop(mut query: Query<&mut Producer>) {
    for mut producer in &mut query {
        producer.as_mut().setup_shop();
    }
}

pub fn consumers_consumer(
    mut consumers: Query<&mut Consumer>,
    mut producers: Query<(Entity, &mut Producer)>,
) {
    for mut consumer in &mut consumers {
        let offers: Vec<Offer> = producers
            .iter()
            .flat_map(|(entity, producer)| producer.get_offers(entity))
            .collect();

        for offer in consumer.choose_offers(offers) {
            let Ok((_, mut producer)) = producers.get_mut(offer.seller) else {
                continue;
            };

            if !consumer.can_afford(offer.price) || !producer.can_sell(offer.clone()) {
                continue;
            }

            consumer.purchase(offer.clone());
            producer.sell(offer);
        }
    }
}

pub fn producers_make_orders() {}

pub fn debug_log(consumers: Query<&Consumer>, producers: Query<&Producer>) {
    for consumer in consumers {
        println!("con: {:#?}", consumer)
    }
    for producer in producers {
        println!("prod: {:#?}", producer)
    }
}
