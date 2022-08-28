/** @type {import('./$types').PageLoad} */
import { error } from "@sveltejs/kit";
export async function load({ fetch }) {
    const res = await fetch('http://127.0.0.1:8080/lfu');
    const res2 = await res.json() ;
    if(res.ok) return { data: res2};
 }
   


  