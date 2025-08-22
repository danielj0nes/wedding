<script>
	let showRsvp = false;
	function toggleRsvp() {
		showRsvp = !showRsvp;
	}
</script>

<main class="page">
	<section class="hero">
		<h1 class="title">Daniel & Hannah</h1>
		<p class="subtitle">We're getting married!</p>
		<p class="date">Saturday, 28th of March, 2026 • 13:00</p>
		<p class="venue">Kirche Stäfa, Kirchbühlstrasse 40, 8712 Stäfa</p>
		<button class="rsvp" on:click={toggleRsvp}>RSVP</button>
	</section>

	{#if showRsvp}
		<section class="rsvp-form">
			<h2>RSVP</h2>
			<form on:submit|preventDefault={() => alert('Thanks — RSVP received!') }>
				<label>
					Your name
					<input required placeholder="Full name" />
				</label>
				<label>
					Attending?
					<select>
						<option value="yes">Yes</option>
						<option value="no">No</option>
					</select>
				</label>
				<div class="actions">
					<button type="submit">Send</button>
					<button type="button" on:click={toggleRsvp}>Close</button>
				</div>
			</form>
		</section>
	{/if}
</main>

<style>
	.page { font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; padding: 2rem; max-width: 720px; margin: 0 auto; }
	.hero { text-align: center; padding: 3rem 1rem; border-radius: 8px; background: linear-gradient(180deg,#fff 0,#f7f7ff 100%); box-shadow: 0 4px 20px rgba(0,0,0,0.05); }
	.title { font-size: 2.25rem; margin: 0; }
	.subtitle { margin: .25rem 0 1rem; color: #666; }
	.date, .venue { margin: 0.25rem 0; color: #333; }
	.rsvp { margin-top: 1rem; padding: .6rem 1rem; background: #0053ba; color: white; border: none; border-radius: 6px; cursor: pointer; }
	.rsvp-form { margin-top: 1.5rem; padding: 1rem; border: 1px solid #eee; border-radius: 6px; }
	label { display: block; margin-bottom: .75rem; }
	input, select { width: 100%; padding: .5rem; border: 1px solid #ddd; border-radius: 4px; }
	.actions { display:flex; gap:.5rem; margin-top:.5rem }
	button[type="submit"] { background:#0b8; border:none; padding:.5rem .75rem; border-radius:4px }
	button[type="button"] { background:#eee; border:none; padding:.5rem .75rem; border-radius:4px }
</style>
