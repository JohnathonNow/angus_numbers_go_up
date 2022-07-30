# ammoios

## Tick order

1. Chunk sync and migration
2. Process environment queue
3. Process NPC queues
4. Process item queues
5. Process player queues
6. Send updates

Each chunk of the world map has a separate set of queues. At the start of each tick,
elements may migrate between queues of different chunks. For example, a player
may move to a new chunk.

Or is it better to have sets of queues per some other sharding scheme, to avoid
migration? Each chunk could have an output buffer to send to nearby players.

## Queue priority

1. Take damage
2. Heal
3. Move
4. Attack
5. Action 



