document.addEventListener('DOMContentLoaded', () => {
    const app = document.getElementById('app');
    const moodIndicator = document.getElementById('mood-indicator');
    const changeMoodBtn = document.getElementById('change-mood');
    const suggestTopicBtn = document.getElementById('suggest-topic');
    const toggleDarkModeBtn = document.getElementById('toggle-dark-mode');
    const chatContainer = document.getElementById('chat-container');
    const messageInput = document.getElementById('message-input');
    const sendButton = document.getElementById('send-button');

    let isDarkMode = false;

    const moods = ['😊', '😎', '🐻', '🍯', '😃', '😅', '😏', '😍', '😇', '😜', '🌞'];
    // const topics = ['!msg Include any contact info: '];

    function changeMood() {
        moodIndicator.textContent = moods[Math.floor(Math.random() * moods.length)];
    }

    function suggestTopic() {
        messageInput.value = '!msg Include any contact info: ';
    }

    function toggleDarkMode() {
        isDarkMode = !isDarkMode;
        document.body.classList.toggle('dark-mode', isDarkMode);
        toggleDarkModeBtn.querySelector('.sun-icon').classList.toggle('hidden', isDarkMode);
        toggleDarkModeBtn.querySelector('.moon-icon').classList.toggle('hidden', !isDarkMode);
    }

    function addMessage(text, sender) {
        const messageElement = document.createElement('div');
        messageElement.classList.add('message', sender === 'user' ? 'user-message' : 'bot-message');
        messageElement.textContent = text;
        chatContainer.appendChild(messageElement);
        chatContainer.scrollTop = chatContainer.scrollHeight;
    }

    function showLoading() {
        const loadingIndicator = document.createElement('div');
        loadingIndicator.id = 'loading-indicator';
        loadingIndicator.innerHTML = '<div class="spinner"></div>';
        chatContainer.appendChild(loadingIndicator);
        chatContainer.scrollTop = chatContainer.scrollHeight;
    }

    function hideLoading() {
        const loadingIndicator = document.getElementById('loading-indicator');
        if (loadingIndicator) {
            loadingIndicator.remove();
        }
    }

    async function handleSendMessage() {
        const message = messageInput.value.trim();
        if (message !== '') {
            addMessage(message, 'user');
            messageInput.value = '';
            showLoading();

            try {
                const response = await fetch('/chat', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ message: message }),
                });

                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }

                const data = await response.json();
                hideLoading();
                addMessage(data.response, 'bot');
            } catch (error) {
                console.error('Error:', error);
                hideLoading();
                addMessage("Sorry, there was an error processing your request.", 'bot');
            }
        }
    }

    changeMoodBtn.addEventListener('click', changeMood);
    suggestTopicBtn.addEventListener('click', suggestTopic);
    toggleDarkModeBtn.addEventListener('click', toggleDarkMode);
    sendButton.addEventListener('click', handleSendMessage);
    messageInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            handleSendMessage();
        }
    });
});
