# Economy simulator idea:

## We simulate consumers and producers, bevy can probably handle the scale but let's see.

## Consumer:

- A consumer has:
  - Wallet (their total money)
  - Map<desire, good>
  - income, monthly

- Fn PurchaseDecision(available_goodies) -> Optional<Goody>

## Producer:

- A Producer has:
  - A Wallet
  - Inventory Map<Goody, count>
  - Production Orders
- Fn ProductPrices() -> Map<Goody, Price>
- Fn CreateOrders();


## Market:

- A Market has:
  - Inventory: List of producers and their inventories
- Fn Purchase(Consumer, Producer)

## Sim loop: 1 update = 1 day

1. If Payday, pay consumers.
2. update_consumers_desires.
2. update_producer_orders.
3. producers_set_prices.
4. consumers_consume.
