<script>
    import { getLogs } from '$lib/api.js';
    import { onMount } from 'svelte';

    let logs = [];
    let continuationToken = 0;
    let showLoadMore = false;
    const page = 20;

    onMount(async () => {
        logs = await loadLogs();
    });

    async function handleLoadMoreClick() {
        const moreLogs = await loadLogs();
        logs.push(moreLogs);
        logs = logs;
    }

    async function loadLogs() {
        const response = await getLogs(page, continuationToken, true);
        continuationToken = response.continuationToken;
        showLoadMore = logs.length > 0 && logs.length % page === 0;

        return response.entities;
    }
</script>

<button type="button" id="btnRefreshLogs" class="btn btn-primary mb-3">Refresh Logs</button>
<div class="table-container">
    <table id="logsTable" class="table table-hover">
        <thead>
            <tr>
                <th>Action</th>
                <th>Timestamp</th>
                <th>EntityId</th>
                <th>EntityType</th>
                <th>Payload</th>
            </tr>
        </thead>
        <tbody>
            {#each logs as log}
                <tr>
                    <td>{log.action}</td>
                    <td>{log.timestamp}</td>
                    <td>{log.entity_id}</td>
                    <td>{log.entity_type}</td>
                    <td>{log.payload}</td>
                </tr>
            {/each}
        </tbody>
    </table>
</div>

{#if showLoadMore}
    <div id="loadMoreLogssBtnContainer">
        <button class="btn btn-link mx-auto d-block mt-3" on:click={handleLoadMoreClick}>Load More</button>
    </div>
{/if}