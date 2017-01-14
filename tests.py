import unittest
import requests


class SlackTestCase(unittest.TestCase):
    def setUp(self):
        # FIXME: tiriamo su il server
        pass

    def test_valid_slack_request(self):
        url = 'http://127.0.0.1:8888/giffetteria'
        data = dict(
            token='gIkuvaNzQIHg97ATvDxqgjtO',
			team_id='T0001',
			team_domain='example',
			channel_id='C2147483705',
			channel_name='test',
			user_id='U2147483697',
			user_name='Steve',
			command='/giffetteria',
			text='ciao',
			response_url='https://hooks.slack.com/commands/1234/5678'
        )
        response = requests.post(url, data)
        self.assertEqual(response.status_code, 200)
        reponse_data = response.json()
        self.assertIn('text', response_data)
        self.assertTrue(response_data['text'])
        self.assertIn('attachments', response_data)
        attachments = response_data['attachments']
        self.assertIn('image_url', attachments)
        self.assertTrue(attachments['image_url'])

    def test_empty_slack_request(self):
        url = 'http://127.0.0.1:8888/giffetteria'
        data = dict(
            token='gIkuvaNzQIHg97ATvDxqgjtO',
			team_id='T0001',
			team_domain='example',
			channel_id='C2147483705',
			channel_name='test',
			user_id='U2147483697',
			user_name='Steve',
			command='/giffetteria',
			text='lkdshflkgfdhgfdhgfdlk',
			response_url='https://hooks.slack.com/commands/1234/5678'
        )
        response = requests.post(url, data)
        self.assertEqual(response.status_code, 404)


if __name__ == '__main__':
    unittest.main()
