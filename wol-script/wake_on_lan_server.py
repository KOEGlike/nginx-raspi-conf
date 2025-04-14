from flask import Flask, jsonify
from wakeonlan import send_magic_packet

app = Flask(__name__)

# Replace with the MAC address of the target PC
TARGET_MAC_ADDRESS = "00:11:22:33:44:55"

@app.route('/start', methods=['GET'])
def wake_pc():
    try:
        # Send the WOL magic packet
        send_magic_packet(TARGET_MAC_ADDRESS)
        return jsonify({"status": "success", "message": "Magic packet sent to wake the PC."}), 200
    except Exception as e:
        return jsonify({"status": "error", "message": str(e)}), 500

if __name__ == '__main__':
    # Run the Flask server on port 5000
    app.run(host='0.0.0.0', port=5000)