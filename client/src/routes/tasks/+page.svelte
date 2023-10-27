<script>
    import TaskRow from "./TaskRow.svelte";
    import CreateTaskModal from "./CreateTaskModal.svelte";
    import { onMount } from "svelte";
    import { getTasks, deleteTask, createTask } from '$lib/api.js';
    import { goto } from "$app/navigation";

    const page = 20;

    let tasks = [];
    let showLoadMore = false;
    let continuationToken = 0;
    let orderBy = "DueDate";
    let descending = true;

    async function loadTasks() {
        const response = await getTasks(page, continuationToken, orderBy, descending);
        continuationToken = response.continuation_token;
        showLoadMore = tasks.length > 0 && tasks.length % page === 0;

        return response.entities;
    }

    onMount(async () => {
        tasks = await loadTasks();
    });

    async function onDeleteTask(event) {
        await deleteTask(event.detail.id);
        tasks = await loadTasks();
    }

    async function onLoadMoreClick() {
        const moreTasks = await loadTasks();
        tasks.push(moreTasks);
        tasks = tasks; // reactivity trigger
    }

    async function changeOrder(field) {
        orderBy = field;
        descending = !descending;
        continuationToken = 0;
        tasks = await loadTasks();
    }

    async function onTaskCreate(event) {
        const task = event.detail.task;
        task.due_date = new Date(Date.parse(task.due_date)).toISOString()
        await createTask(task.summary, task.priority, task.status, task.description, task.due_date);
        continuationToken = 0;
        tasks = await loadTasks();
    }
</script>


<button type="button" class="btn btn-primary mb-3" data-bs-toggle="modal" data-bs-target="#createTaskModal">Create Task</button>
<div class="table-container">
    <table id="taskTable" class="table table-hover">
        <thead>
            <tr>
                <th 
                    class:sorted-asc={orderBy === "Summary" && !descending}
                    class:sorted-desc={orderBy === "Summary" && descending}
                    on:click={async () => changeOrder("Summary")}
                >
                    Summary
                </th>
                <th
                    class:sorted-asc={orderBy === "Priority" && !descending}
                    class:sorted-desc={orderBy === "Priority" && descending}
                    on:click={async () => changeOrder("Priority")}
                >
                    Priority
                </th>
                <th
                    class:sorted-asc={orderBy === "Status" && !descending}
                    class:sorted-desc={orderBy === "Status" && descending}
                    on:click={async () => changeOrder("Status")}
                >
                    Status
                </th>
                <th 
                    class:sorted-asc={orderBy === "CreateDate" && !descending}
                    class:sorted-desc={orderBy === "CreateDate" && descending}
                    on:click={async () => changeOrder("CreateDate")}
                >
                    Created
                </th>
                <th
                    class:sorted-asc={orderBy === "DueDate" && !descending}
                    class:sorted-desc={orderBy === "DueDate" && descending}
                    on:click={async () => changeOrder("DueDate")}
                >
                    Due
                </th>
            </tr>
        </thead>
        <tbody>
            {#each tasks as task}
                <TaskRow {task} on:delete={onDeleteTask} on:click={() => goto('tasks/' + task.id)} />
            {/each}
        </tbody>
    </table>
</div>

{#if showLoadMore}
    <button class="btn btn-link mx-auto d-block mt-3" on:click={onLoadMoreClick}>Load More</button>
{/if}

<CreateTaskModal on:create={onTaskCreate} />

<style>
.sorted-asc::after {
    content: " ↑";
}
.sorted-desc::after {
    content: " ↓";
}
</style>