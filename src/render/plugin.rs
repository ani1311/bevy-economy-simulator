use bevy::prelude::*;
use egui_plot::{Line, Plot, PlotPoints};
use std::collections::VecDeque;

use bevy::{
    app::{FixedUpdate, Plugin, PreStartup},
    camera::Camera2d,
    ecs::{resource::Resource, schedule::IntoScheduleConfigs, system::Commands},
};
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, EguiStartupSet, egui};

use crate::{Consumer, Producer, debug_log, resources::WorldState};

pub struct EconomyDisplayPlugin;

#[derive(Clone)]
struct MarketSnapshot {
    day: u32,
    average_consumer_cash: f64,
    producer_cash: f64,
    inventory: f64,
    price: f64,
}

#[derive(Resource, Default, Clone)]
struct MarketHistory {
    snapshots: VecDeque<MarketSnapshot>,
}

impl Plugin for EconomyDisplayPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<MarketHistory>()
            .add_systems(
                PreStartup,
                setup_display.before(EguiStartupSet::InitContexts),
            )
            .add_systems(FixedUpdate, record_market_history.after(debug_log))
            .add_systems(EguiPrimaryContextPass, draw_dashboard);
    }
}

fn setup_display(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn record_market_history(
    world: Res<WorldState>,
    consumers: Query<&Consumer>,
    producers: Query<&Producer>,
    mut history: ResMut<MarketHistory>,
) {
    if history
        .snapshots
        .back()
        .is_some_and(|s| s.day == world.today())
    {
        return;
    }

    let consumer_count = consumers.iter().count();

    let total_consumer_cash: u64 = consumers
        .iter()
        .map(|consumer| consumer.wallet_balance() as u64)
        .sum();

    let average_consumer_cash = if consumer_count == 0 {
        0.0
    } else {
        total_consumer_cash as f64 / consumer_count as f64
    };

    let producer_cash: u64 = producers
        .iter()
        .map(|producer| producer.wallet_balance() as u64)
        .sum();

    let inventory: u64 = producers
        .iter()
        .filter_map(|producer| producer.keyboard_inventory())
        .map(|price| price as u64)
        .sum();

    let prices: Vec<f64> = producers
        .iter()
        .filter_map(|producer| producer.keyboard_price())
        .map(|price| price as f64)
        .collect();

    let average_price = if prices.is_empty() {
        0.0
    } else {
        prices.iter().sum::<f64>() / prices.len() as f64
    };

    history.snapshots.push_back(MarketSnapshot {
        day: world.today(),
        average_consumer_cash,
        producer_cash: producer_cash as f64,
        inventory: inventory as f64,
        price: average_price,
    });
}

fn draw_dashboard(mut contexts: EguiContexts, history: Res<MarketHistory>) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "economy_dashboard".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    draw_header(&mut viewport_ui, &history);
    draw_market_graphs(&mut viewport_ui, &history);
    draw_market_sidebar(&mut viewport_ui, &history);

    egui::CentralPanel::default().show(&mut viewport_ui, |ui| {
        ui.heading("Dashboard");
        ui.separator();
        ui.label("Add graphs and simulation controls here.");
        ui.label(format!("Recorded samples: {}", history.snapshots.len()));
    });

    Ok(())
}

fn draw_header(viewport_ui: &mut egui::Ui, history: &MarketHistory) {
    egui::Panel::top("dashboard_header").show(viewport_ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Economy Simulator");

            ui.separator();

            match history.snapshots.back() {
                Some(snapshot) => {
                    ui.label(format!("Day {}", snapshot.day));
                }
                None => {
                    ui.label("Waiting for data");
                }
            }
        });
    });
}

fn draw_market_sidebar(viewport_ui: &mut egui::Ui, history: &MarketHistory) {
    egui::Panel::left("dashboard_sidebar")
        .default_size(220.0)
        .resizable(true)
        .show(viewport_ui, |ui| {
            ui.heading("Market");
            ui.separator();

            let Some(snapshot) = history.snapshots.back() else {
                ui.label("Waiting for market data...");
                return;
            };

            egui::Grid::new("Market stats")
                .num_columns(2)
                .spacing([16.0, 8.0])
                .show(ui, |ui| {
                    ui.label(format!("Price: ${:.2}", snapshot.price));
                    ui.label(format!("Inventory: {:.0}", snapshot.inventory));
                    ui.label(format!("Producer cash: ${:.2}", snapshot.producer_cash));
                    ui.label(format!(
                        "Average consumer cash: ${:.2}",
                        snapshot.average_consumer_cash
                    ));
                });
        });
}
fn draw_market_graphs(viewport_ui: &mut egui::Ui, history: &MarketHistory) {
    egui::CentralPanel::default().show(viewport_ui, |ui| {
        ui.heading("Market History");
        ui.separator();

        if history.snapshots.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Graph will appear");
            });

            return;
        }

        draw_price_graph(ui, history);
        ui.add_space(12.0);
        draw_inventory_graph(ui, history);
        ui.add_space(12.0);
        draw_cash_graph(ui, history);
    });
}

fn draw_price_graph(ui: &mut egui::Ui, history: &MarketHistory) {
    let points = PlotPoints::from_iter(history.snapshots.iter().map(|s| [s.day as f64, s.price]));
    ui.label("Keyboard price");

    Plot::new("price_graph")
        .height(180.0)
        .x_axis_label("Day")
        .y_axis_label("Price")
        .allow_drag(false)
        .allow_scroll(false)
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Price", points)
                    .color(egui::Color32::LIGHT_GREEN)
                    .width(2.0),
            )
        });
}
fn draw_inventory_graph(ui: &mut egui::Ui, history: &MarketHistory) {
    let points = PlotPoints::from_iter(
        history
            .snapshots
            .iter()
            .map(|s| [s.day as f64, s.inventory]),
    );
    ui.label("Producer inventory");

    Plot::new("inventory_graph")
        .height(180.0)
        .x_axis_label("Day")
        .y_axis_label("Units")
        .allow_drag(false)
        .allow_scroll(false)
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Inventory", points)
                    .color(egui::Color32::LIGHT_BLUE)
                    .width(2.0),
            )
        });
}
fn draw_cash_graph(viewport_ui: &mut egui::Ui, history: &MarketHistory) {}
