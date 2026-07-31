# Simple Agent-Based Keyboard Market

## Overview

The simulation is an agent-based economy built in Rust using Bevy ECS.

The first version contains:

* One good: keyboards
* Consumers
* Keyboard producers
* A market that processes purchases
* Money, inventory, demand, pricing, and production delays

## Consumers

Each consumer is a Bevy entity with:

* A wallet containing a limited amount of money
* A desire value representing how much they want a keyboard
* A keyboard inventory
* Periodic income or salary

A consumer purchases a keyboard only when:

1. Their desire is high enough.
2. They can afford the producer's price.
3. A keyboard is available.

The purchase decision should not be purely random. Randomness can slightly change desire over time, but the final decision should depend on desire, price, and available money.

A simple rule is:

```text
willingness_to_pay = base_value × desire

Buy when:
wallet >= price
and willingness_to_pay >= price
```

After buying a keyboard, the consumer's desire should decrease.

## Producers

Each producer has:

* A wallet
* A keyboard inventory
* A posted selling price
* A production cost
* A production batch size
* A production duration
* A target inventory level

The producer decides when to begin production based on its current inventory and available cash.

Production is not immediate. The producer creates a production job that takes several ticks to complete.

```text
Start production when:
inventory < target inventory
and no production job is active
and wallet >= production cost
```

When the job finishes, the completed keyboards are added to the producer's inventory.

## Pricing

The producer sets the keyboard price.

The producer can adjust the price using simple feedback:

```text
If keyboards sell out:
    increase the price

If inventory keeps accumulating:
    decrease the price
```

Consumers decide whether the posted price is acceptable based on their desire and willingness to pay.

## Market

The market is a Bevy resource or collection of systems that processes transactions.

The market checks:

* Whether the consumer has enough money
* Whether the producer has enough inventory
* Whether the consumer is willing to pay the price

When a purchase succeeds:

```text
consumer wallet -= price
producer wallet += price

consumer keyboard inventory += 1
producer keyboard inventory -= 1
```

The market coordinates trades but does not make decisions for consumers or producers.

## Simulation Tick

Each simulation tick follows this general order:

1. Pay consumer income.
2. Update consumer desire.
3. Let producers start production jobs.
4. Advance active production jobs.
5. Complete finished production jobs.
6. Let producers adjust prices.
7. Let consumers make purchase decisions.
8. Process valid purchases.
9. Record market statistics.

## Core Economic Loop

```text
Consumers receive income
        ↓
Consumer desire increases
        ↓
Consumers buy keyboards
        ↓
Producers receive revenue
        ↓
Producers pay production costs
        ↓
Production takes several ticks
        ↓
New keyboards enter inventory
```

This simple loop can already produce shortages, excess inventory, changing prices, delayed purchases, and producers running low on capital.
