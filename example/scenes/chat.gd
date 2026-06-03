extends Panel

@onready var editor: TextEdit = $TextEdit
@onready var feed: RichTextLabel = $RichTextLabel

func add_message(sender_name: String, message: String):
	feed.add_text(sender_name + ": " + message + "\n")
	
func _on_text_edit_gui_input(event):
	if event is InputEventKey and event.keycode == KEY_ENTER and event.pressed:
		var text = editor.text.strip_edges()
		if text.length() > 0:
			Game.send_packet({"Chat": {"message": text}})
		editor.clear()
		editor.release_focus()
