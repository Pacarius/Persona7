curl -X POST http://192.168.50.84:11434/api/generate -H "Content-Type: application/json" -d @- << 'EOF'
{
  "context": null,
  "format": {
    "properties": {
      "sleep_time": {
        "type": "string"
      },
      "wake_time": {
        "type": "string"
      }
    },
    "required": ["wake_time", "sleep_time"],
    "type": "object"
  },
  "images": null,
  "keep_alive": null,
  "model": "llama3.2",
  "options": null,
  "prompt": "Man is 22. Man is Creative,Resourceful,Independent, Laid-back,Ambitious,Practical. He spends his free time tinkering in his DIY workshop, experimenting with woodworking projects and home brewing, and dreaming of one day opening his own craft brewery or furniture making business. \n\n    Daily Plan: Today, Man is planning to do the following five things: []\n    Return Man's waking and sleeping time, in HH:MM:SS form.",
  "raw": null,
  "stream": false,
  "suffix": null,
  "system": null,
  "template": null
}
EOF