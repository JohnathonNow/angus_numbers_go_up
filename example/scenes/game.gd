extends Node

const PORT : int = 3030

@onready var main : Node = get_tree().root.get_node("Main")
@onready var players : Node = main.get_node("Players")

var menu : Control = null
var map : Node = null
var player_name : String = ""

var socket : WebSocketPeer = WebSocketPeer.new()
var is_connected_to_server : bool = false

func _ready():
	call_deferred("spawn_menu")

func spawn_menu():
	menu = preload("res://scenes/menu.tscn").instantiate()
	main.add_child(menu)

func _process(_delta):
	socket.poll()
	var state = socket.get_ready_state()
	
	if state == WebSocketPeer.STATE_OPEN:
		if not is_connected_to_server:
			is_connected_to_server = true
			send_packet({"Login": {"username": player_name}})
			call_deferred("load_map")

		while socket.get_available_packet_count():
			var packet = socket.get_packet()
			var text = packet.get_string_from_utf8()
			var json = JSON.new()
			var error = json.parse(text)
			if error == OK:
				var data = json.get_data()
				handle_packet(data)
			else:
				print("JSON Parse Error: ", json.get_error_message())

	elif state == WebSocketPeer.STATE_CLOSED:
		if is_connected_to_server:
			is_connected_to_server = false
			print("Disconnected from server.")

func connect_to_server():
	var err = socket.connect_to_url("ws://127.0.0.1:%d/chat" % PORT)
	if err != OK:
		print("Unable to connect")

func send_packet(data: Dictionary):
	var json = JSON.stringify(data)
	socket.send_text(json)

func handle_packet(data: Dictionary):
	if data.has("Chat"):
		var chat_data = data["Chat"]
		if map and map.has_node("Chat"):
			map.get_node("Chat").add_message(chat_data["sender"], chat_data["message"])
	elif data.has("State"):
		var state_data = data["State"]
		var players_data = state_data["players"]

		var current_player_usernames = []

		for p_data in players_data:
			var username = p_data["username"]
			current_player_usernames.append(username)

			var player_node = players.get_node_or_null(username)
			if not player_node:
				player_node = spawn_player(username)

			if player_node:
				player_node.target_x = p_data["x"]
				player_node.target_y = p_data["y"]
				player_node.hp = p_data["hp"]
				player_node.max_hp = p_data["max_hp"]
				player_node.xp = p_data["xp"]
				player_node.level = p_data["level"]
				player_node.update_ui()

		# Remove players that are no longer in the state
		for child in players.get_children():
			if not current_player_usernames.has(child.name):
				child.queue_free()

func load_map():
	# Free old stuff.
	if map != null:
		map.queue_free()
	if menu != null:
		menu.queue_free()
	
	# Spawn map.
	map = preload("res://scenes/map.tscn").instantiate()
	main.add_child(map)

func spawn_player(username: String):
	var player = preload("res://scenes/player.tscn").instantiate()
	player.name = username
	player.peer_name = username
	players.add_child(player, true)
	return player

func remove_player(username: String):
	if not players.has_node(username):
		return
	players.get_node(username).queue_free()

func get_player_name(username: String) -> String:
	return username
