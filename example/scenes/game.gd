# Autoloaded script/singleton
# Added to the root location
# Manages game states
extends Node

const PORT : int = 13370

@onready var main : Node = get_tree().root.get_node("Main")
@onready var players : Node = main.get_node("Players")

var menu : Control = null
var map : Node = null
var player_name : String = ""


func _ready():
	menu = preload("res://scenes/menu.tscn").instantiate()
	main.add_child(menu)
	
	# if multiplayer.is_server():
	multiplayer.peer_connected.connect(spawn_player)
	multiplayer.peer_disconnected.connect(remove_player)

func load_map():
	# Free old stuff.
	if map != null:
		map.queue_free()
	if menu != null:
		menu.queue_free()
	
	# Spawn map.
	map = preload("res://scenes/map.tscn").instantiate()
	main.add_child(map)
	
	if not multiplayer.is_server(): #do not spawn a player for the server
		spawn_player(multiplayer.get_unique_id()).peer_name = player_name

func spawn_player(id: int):
	if id == 1: #do not spawn a player for the server
		return
	var player = preload("res://scenes/player.tscn").instantiate()
	player.peer_id = id
	players.add_child(player, true)
	return player

func remove_player(id: int):
	if not players.has_node(str(id)):
		return
	players.get_node(str(id)).queue_free()
	
func get_player_name(id: int) -> String:
	if not players.has_node(str(id)):
		return ""
	return players.get_node(str(id)).peer_name
