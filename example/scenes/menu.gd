# Main menu
extends Control

func _ready():
	if OS.has_feature("dedicated_server"):
		var peer = WebSocketMultiplayerPeer.new()
		print(peer.create_server(Game.PORT, "0.0.0.0"))
		multiplayer.multiplayer_peer = peer
		multiplayer.get_unique_id()
		Game.load_map()

func _on_connection_established(_id: int):
	# needed to satisfy
	Game.player_name = $"VBoxContainer/Name".text
	Game.load_map()


func _on_connect_pressed():
	var peer = WebSocketMultiplayerPeer.new()
	print(peer.create_client("ws://127.0.0.1:%d" % Game.PORT))
	#peer.create_client("localhost", Game.PORT)
	multiplayer.multiplayer_peer = peer
	peer.connect(&"peer_connected", _on_connection_established.bind())
