# Main menu
extends Control

func _ready():
	if OS.has_feature("dedicated_server"):
		# Dedicated server mode not supported with raw websocket in godot client yet
		pass

func _on_connection_established():
	Game.player_name = $"VBoxContainer/Name".text
	Game.load_map()


func _on_connect_pressed():
	Game.player_name = $"VBoxContainer/Name".text
	Game.connect_to_server()
