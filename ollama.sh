curl -X POST http://localhost:11434/api/chat -H "Content-Type: application/json" -d '{
  "model": "llama3.1",
  "messages": [{"role": "user", "content": "Man is 22. Man is Creative,Resourceful,Independent, Laid-back,Ambitious,Practical. He spends his free time tinkering in his DIY workshop, experimenting with woodworking projects and home brewing, and dreaming of one day opening his own craft brewery or furniture making business. \n\n            Today is January 1.\n            Man's day starts at 06:00:00 and ends at 22:30:0\n            Here is a list of Man's plans today in broad strokes in the format of a list of (Action_Description, Start_Time(HH:MM:SS form), End_Time(HH:MM:SS form))\n"}],
  "stream": false,
  "format": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "capital": {
        "type": "string"
      },
      "languages": {
        "type": "array",
        "items": {
          "type": "string"
        }
      }
    },
    "required": [
      "name",
      "capital", 
      "languages"
    ]
  }
}'