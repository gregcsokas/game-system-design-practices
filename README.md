# Game Systems Practices with Bevy

### Really Simple Health System 
Folder: `simple_health_system`

Basically a Health component with `max_health` and `current_health` fields.
and some utility functions like `take_damage`, `heal`, `get_health`, `is_dead`.

### Really Simple Health System V2
Folder: `simple_health_system_v2`

A little more extra functionality, and it's event based now.
With a dedicated `Damage` component. 
It sends a `HitEvent` and the target will get the damage.