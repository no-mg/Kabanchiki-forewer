document.addEventListener('DOMContentLoaded', function() {
    const apiButton = document.getElementById('api-button');
    const apiResponse = document.getElementById('api-response');
    
    apiButton.addEventListener('click', function() {
        // Показываем состояние загрузки
        apiResponse.textContent = 'Загрузка...';
        apiResponse.className = 'api-response loading';
        
        // Делаем запрос к API
        fetch('/api/hello')
            .then(response => {
                if (!response.ok) {
                    throw new Error('Ошибка сети: ' + response.status);
                }
                return response.json();
            })
            .then(data => {
                // Отображаем успешный ответ
                apiResponse.textContent = JSON.stringify(data, null, 2);
                apiResponse.className = 'api-response success';
            })
            .catch(error => {
                // Отображаем ошибку
                apiResponse.textContent = 'Ошибка: ' + error.message;
                apiResponse.className = 'api-response error';
            });
    });
});