const BASE_URL = 'http://localhost:3005/api/tasks';

async function getErrorContent(response) {
    try {
        const contentType = response.headers.get('content-type');
        if (contentType && contentType.includes('application/json')) {
            // Try to parse the error content as JSON
            const jsonError = await response.json();
            return JSON.stringify(jsonError);
        } else {
            // Treat it as plain text
            return await response.text();
        }
    } catch (error) {
        return '';
    }
}

export async function getTasks(take, continuationToken, orderBy, descendingSort) {
    const url = `${BASE_URL}?take=${take}&continuation_token=${continuationToken}&order_by=${orderBy}&descending_sort=${descendingSort}`;
    var response = await fetch(url);

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }

    return await response.json();
}

export async function getLogs(take, continuationToken, descending) {
    const url = `${BASE_URL}/logs?take=${take}&continuation_token=${continuationToken}&descending=${descending}`;
    var response = await fetch(url);

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }

    return await response.json();
}

export async function getTaskDetails(taskId) {
    const url = `${BASE_URL}/${taskId}`;
    var response = await fetch(url);

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }

    return await response.json();
}

export async function getSearchResults(phrase) {
    const url = `${BASE_URL}/search/${phrase}?continuation_token=0&take=10`;
    var response = await fetch(url);

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }

    return await response.json();
}

export async function updateTaskRoot(taskId, root_id) {
    const url = `${BASE_URL}/${taskId}/root`;
    var response = await fetch(url, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ root_id })
    });

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }
}

export async function createTask(summary, priority, status, description, due_date) {
    const url = `${BASE_URL}`;
    var response = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ summary, priority, status, description, due_date })
    });

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }
}

export async function updateTask(taskId, summary, priority, status, description, due_date) {
    const url = `${BASE_URL}/${taskId}`;
    var response = await fetch(url, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ summary, priority, status, description, due_date })
    });

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }
}

export async function deleteTask(taskId) {
    const url = `${BASE_URL}/${taskId}`;
    var response = await fetch(url, {
        method: 'DELETE'
    });

    if (!response.ok) {
        const errorContent = await getErrorContent(response);
        throw new Error(`Error: ${response.status} - ${errorContent}`);
    }
}
