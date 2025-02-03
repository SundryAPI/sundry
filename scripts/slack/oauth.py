from flask import Flask, redirect, request, url_for
import requests
import os
from dotenv import load_dotenv
from flask_sqlalchemy import SQLAlchemy
from datetime import datetime


# Load .env file
load_dotenv()


app = Flask(__name__)
app.config['SQLALCHEMY_DATABASE_URI'] = os.getenv('DATABASE_URL')  # Set this in your .env file
app.config['SQLALCHEMY_TRACK_MODIFICATIONS'] = False
db = SQLAlchemy(app)


# Slack App Credentials
CLIENT_ID = os.environ.get('SLACK_CLIENT_ID')
CLIENT_SECRET = os.environ.get('SLACK_CLIENT_SECRET')
REDIRECT_URI = 'https://127.0.0.1:5000/callback'
SCOPES = 'channels:read channels:join groups:read im:read mpim:read users:read channels:history groups:history im:history mpim:history'


class Organization(db.Model):
    __tablename__ = 'organizations'
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(255), nullable=False)
    created_at = db.Column(db.TIMESTAMP, default=datetime.utcnow)
    updated_at = db.Column(db.TIMESTAMP, default=datetime.utcnow, onupdate=datetime.utcnow)
    deleted_at = db.Column(db.TIMESTAMP)


class AppInstallation(db.Model):
    __tablename__ = 'app_installations'
    __table_args__ = {'schema': 'slack'}
    id = db.Column(db.BigInteger, primary_key=True)
    access_token = db.Column(db.Text, nullable=False, unique=True)
    organization_id = db.Column(db.Integer, db.ForeignKey('organizations.id'))
    created_at = db.Column(db.TIMESTAMP, server_default=db.func.current_timestamp())
    deleted_at = db.Column(db.TIMESTAMP)
    organization = db.relationship('Organization', backref=db.backref('installations', lazy=True))


@app.route('/')
def index():
    # Redirect to Slack for authorization
    auth_url = f"https://slack.com/oauth/v2/authorize?client_id={CLIENT_ID}&scope={SCOPES}&redirect_uri={REDIRECT_URI}"
    return redirect(auth_url)


@app.route('/callback')
def callback():
    # Handle the callback from Slack
    code = request.args.get('code')
    if code:
        # Exchange code for token
        token_response = requests.post('https://slack.com/api/oauth.v2.access', {
            'client_id': CLIENT_ID,
            'client_secret': CLIENT_SECRET,
            'code': code,
            'redirect_uri': REDIRECT_URI
        }).json()
        
        if token_response.get('ok'):
            access_token = token_response['access_token']

            # Add organization
            new_organization = Organization(name=f"Slack_{access_token[:10]}")
            db.session.add(new_organization)
            db.session.flush()  # To get the ID before committing
            
            # Add installation with the new organization's ID
            new_installation = AppInstallation(access_token=access_token, organization_id=new_organization.id)
            db.session.add(new_installation)
            db.session.commit()
            
            return f"Token received: {access_token}. Success!"
        else:
            return f"Error getting token: {token_response['error']}"
    else:
        return "No Code Received"


if __name__ == '__main__':
    print("This web server at / will redirect to Slack. Provide permissions then change the callback from https to http for the token")
    app.run(debug=True)
