extends Panel

@onready var editor: TextEdit = $TextEdit
@onready var feed: RichTextLabel = $RichTextLabel

@rpc("any_peer", "call_local", "reliable")
func chat(message: String):
	var sender_id = multiplayer.get_remote_sender_id()
	var sender_name = Game.get_player_name(sender_id)
	if not multiplayer.is_server(): 
		feed.add_text(sender_name + ": " + message + "\n")
	
func _on_text_edit_gui_input(event):
	if event is InputEventKey and event.keycode == KEY_ENTER:
		chat.rpc(editor.text)
		editor.clear()
		editor.release_focus()
