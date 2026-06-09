extends CharacterBody3D

const SPEED = 15.0

@export var peer_name : String : 
	set(value):
		peer_name = value
		update_ui()

var hp: int = 100
var max_hp: int = 100
var xp: int = 0
var level: int = 1

var target_x: int = 0
var target_y: int = 0

func update_ui():
	$Label3D.text = "%s\nLvl: %d XP: %d\nHP: %d/%d" % [peer_name, level, xp, hp, max_hp]

func _ready():
	# Only enable camera for the current player
	$Camera3D.current = (peer_name == Game.player_name)
	update_ui()

func _process(delta):
	# Interpolate position towards the target coordinates received from backend
	# In the backend, 'x' and 'y' are grid coordinates. We'll map them directly to X and Z in 3D.
	var target_pos = Vector3(float(target_x), position.y, float(target_y))
	position = position.lerp(target_pos, delta * 10.0)

func _input(event):
	if peer_name != Game.player_name:
		return

	if event is InputEventMouseButton and event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
		# Ensure we don't move when clicking on UI
		if Input.mouse_mode == Input.MOUSE_MODE_VISIBLE:
			var camera = get_viewport().get_camera_3d()
			if not camera:
				return

			var from = camera.project_ray_origin(event.position)
			var to = from + camera.project_ray_normal(event.position) * 1000.0

			var space_state = get_world_3d().direct_space_state
			var query = PhysicsRayQueryParameters3D.create(from, to)
			var result = space_state.intersect_ray(query)

			if result:
				var hit_pos = result.position
				var grid_x = int(round(hit_pos.x))
				var grid_y = int(round(hit_pos.z))
				Game.send_packet({"Walk": {"x": grid_x, "y": grid_y}})
			else:
				# Fallback: intersect with Y=0 plane
				var plane = Plane(Vector3.UP, 0)
				var intersection = plane.intersects_ray(from, camera.project_ray_normal(event.position))
				if intersection:
					var grid_x = int(round(intersection.x))
					var grid_y = int(round(intersection.z))
					Game.send_packet({"Walk": {"x": grid_x, "y": grid_y}})
