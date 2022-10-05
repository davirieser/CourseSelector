<script lang="ts">
	import Spinner from '$components/spinner.svelte';
    import { getCourse } from "$src/routes/+page";
    import { selectedCourses } from "$lib/stores/selectedCourses";
    export let selectedCourseVarationID = 0;
    export let selectedCourseName = "";
    $: console.log($selectedCourses);

    let localCourse = "";
    $: console.log(localCourse);
</script>

{#await getCourse(selectedCourseVarationID, selectedCourseName)}
    <div class="flex justify-center mx-auto">
        <Spinner />
    </div>
{:then data} 
    <div class="w-auto">
        <table class="table w-auto">
            <thead>
                <tr>
                    <th>Group</th>
                    <th>Date</th>
                    <th>Time</th>
                    <th>Location</th>
                    <th>Add to calender</th>

                </tr>
            </thead>
            <tbody>
                {#each data.times as data, i}
                    <tr>
                        <th>{data.group}</th>
                        <td>{data.date}</td>
                        <td>{data.time}</td>
                        <td>{data.location}</td>
                        <td>
                            {#if data.newGroup}
                                <input type="radio" name="course" value={data.start + "," + data.end + "," + data.title + "," + data.group} bind:group={localCourse}
                                    class="bg-slate-800 rounded focus:ring-blue-800 hover:ring-blue-800"/>
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/await}