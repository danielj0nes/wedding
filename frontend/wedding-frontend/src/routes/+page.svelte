<script>
	let showRsvp = false;
	function toggleRsvp() {
		showRsvp = !showRsvp;
	}

	/**
     * @param {{ preventDefault: () => void; currentTarget: any; }} event
     */
	async function handleSubmit(event) {
		event.preventDefault();
		const form = event.currentTarget;
		if (!(form instanceof HTMLFormElement)) return;
		const formData = new FormData(form);
		const data = {
			name: formData.get('name'),
			email: formData.get('email'),
			attending: formData.get('attending'),
			comments: formData.get('comments') || null,
		};
		try {
			const response = await fetch('/register', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json',
				},
				body: JSON.stringify(data),
			});
			if (response.ok) {
				alert('Thanks — RSVP received!');
				toggleRsvp(); // Close the modal
			} else {
				alert('Error submitting RSVP');
			}
		} catch (error) {
			console.error('Error:', error);
			alert('Error submitting RSVP');
		}
	}

	// Countdown timer
	let countdown = { days: 0, hours: 0, minutes: 0, seconds: 0 };
	
	function updateCountdown() {
		const weddingDate = new Date('2026-03-28T11:00:00Z');
		const now = new Date();
		const diff = weddingDate.getTime() - now.getTime();
		
		if (diff > 0) {
			countdown.days = Math.floor(diff / (1000 * 60 * 60 * 24));
			countdown.hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
			countdown.minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
			countdown.seconds = Math.floor((diff % (1000 * 60)) / 1000);
		} else {
			countdown = { days: 0, hours: 0, minutes: 0, seconds: 0 };
		}
	}
	
	// Update countdown every second
	import { onMount } from 'svelte';
	onMount(() => {
		updateCountdown();
		const interval = setInterval(updateCountdown, 1000);
		return () => clearInterval(interval);
	});
</script>

<svelte:head>
	<title>Daniel & Hannah's Wedding</title>
</svelte:head>

<main class="page">
	<div class="banner-wrapper">
		<img src="/images/staefa_kirche_main_1.jpg" alt="Wedding venue - Kirche Stäfa" class="banner-image" />
		<div class="top-date" aria-hidden="true">- 28.03.2026 -</div>
		<div class="countdown-container">
			<div class="countdown-item">
				<span class="countdown-number">{countdown.days}</span>
				<span class="countdown-label">Days</span>
			</div>
			<div class="countdown-item">
				<span class="countdown-number">{countdown.hours}</span>
				<span class="countdown-label">Hours</span>
			</div>
			<div class="countdown-item">
				<span class="countdown-number">{countdown.minutes}</span>
				<span class="countdown-label">Minutes</span>
			</div>
			<div class="countdown-item">
				<span class="countdown-number">{countdown.seconds}</span>
				<span class="countdown-label">Seconds</span>
			</div>
		</div>
		<div class="banner-text">
			<h1 class="title">Daniel & Hannah</h1>
			<p class="subtitle">We're getting married!</p>
			
			<p class="venue"><a href="https://maps.app.goo.gl/BYGnX4iWFiuQCn3XA" target="_blank" rel="noopener noreferrer">Kirche Stäfa, Kirchbühlstrasse 40, 8712 Stäfa</a></p>
			<button class="rsvp" on:click={toggleRsvp}>RSVP</button>
		</div>
	</div>

	<!-- RSVP Modal Overlay -->
	{#if showRsvp}
		<div class="rsvp-overlay" role="dialog" aria-modal="true" tabindex="-1" on:click={toggleRsvp} on:keydown={(e) => { if (e.key === 'Escape') toggleRsvp(); }}>
			<section id="rsvp-section" class="rsvp-modal" role="dialog" tabindex="-1" on:click|stopPropagation on:keydown|stopPropagation>
				<div class="rsvp-header">
					<h2>RSVP</h2>
					<button class="close-button" on:click={toggleRsvp} aria-label="Close RSVP form">×</button>
				</div>
				<form on:submit={handleSubmit}>
					<div class="form-group">
						<label for="name">
							<span class="label-text">Name(s)</span>
							<input id="name" name="name" required placeholder="Enter name(s)" />
						</label>
					</div>
					<div class="form-group">
						<label for="email">
							<span class="label-text">Your email</span>
							<input id="email" name="email" type="email" required placeholder="Email address" />
						</label>
					</div>
					<div class="form-group">
						<span class="label-text">Attending?</span>
						<div class="radio-group">
							<label class="radio-label">
								<input type="radio" name="attending" value="yes" required />
								<span class="custom-radio"></span>
								Yes
							</label>
							<label class="radio-label">
								<input type="radio" name="attending" value="no" required />
								<span class="custom-radio"></span>
								No
							</label>
						</div>
					</div>
					<div class="form-group">
						<label for="comments">
							<span class="label-text">Comments (e.g., food allergies, special requests)</span>
							<textarea id="comments" name="comments" placeholder="Enter any comments here..." rows="6"></textarea>
						</label>
					</div>

					<div class="actions">
						<button type="button" class="cancel-btn" on:click={toggleRsvp}>Cancel</button>
						<button type="submit" class="submit-btn">Send RSVP</button>
					</div>
				</form>
			</section>
		</div>
	{/if}

	<div class="content">
	</div>

	</main>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800&family=Playfair+Display:wght@400;500;600;700;800;900&family=Great+Vibes&family=Cinzel:wght@400;500;600;700;800;900&family=Allura&display=swap');

	:global(html, body) { 
		margin: 0; 
		padding: 0; 
		overflow: hidden;
		scroll-behavior: smooth;
	}

	:root { 
		--page-max-width: 960px;
		--accent: #e91e63;
	}

	:global(body) {
		font-family: 'Inter', system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial;
		color: #111;
	}
	:global(h1, h2, h3, .title) { 
		font-family: 'Playfair Display', serif; 
	}

	:global(::selection) {
		background-color: #fce4ec;
		color: #333;
	}
	:global(::-moz-selection) {
		background-color: #fce4ec;
		color: #333;
	}

	.page { 
		font-family: system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial; 
		padding: 0; 
	}

	.banner-wrapper { 
		position: relative; 
		width: 100vw; 
		overflow: hidden; 
	}
	.banner-wrapper::before { 
		content: ""; 
		position: absolute; 
		inset: 0; 
		background: linear-gradient(180deg, rgba(0,0,0,0.6) 0%, rgba(0,0,0,0.35) 40%, rgba(0,0,0,0.2) 100%); 
		z-index: 0; 
		pointer-events: none; 
	}
	.banner-image { 
		width: 100%; 
		height: 100vh; 
		object-fit: cover; 
		display: block; 
	}

	.banner-text { 
		position: absolute; 
		inset: 0; 
		display: flex; 
		flex-direction: column; 
		align-items: center; 
		justify-content: center; 
		color: white; 
		text-align: center; 
		padding: 2rem; 
		z-index: 1; 
	}
	.banner-text .title { 
		font-family: 'Playfair Display', serif;
		font-size: clamp(2.5rem, 8vw, 5rem);
		font-weight: 800;
		letter-spacing: 0.08em;
		margin: 0;
		line-height: 1.1;
		background: linear-gradient(135deg, #ffffff 0%, #f8f9ff 50%, #ffffff 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		position: relative;
	}

	.banner-text .title::selection {
		-webkit-background-clip: initial;
		-webkit-text-fill-color: initial;
		background-clip: initial;
		background: #fce4ec;
		color: #333;
		text-shadow: none;
	}

	.banner-text .title::-moz-selection {
		-webkit-background-clip: initial;
		-webkit-text-fill-color: initial;
		background-clip: initial;
		background: #fce4ec;
		color: #333;
		text-shadow: none;
	}
	
	.banner-text .title::after {
		content: '';
		position: absolute;
		bottom: -15px;
		left: 50%;
		transform: translateX(-50%);
		width: 120px;
		height: 3px;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.8), transparent);
		border-radius: 2px;
		box-shadow: 0 2px 10px rgba(255,255,255,0.3);
	}
	.banner-text .subtitle { 
		margin-top: 2rem;
		margin-bottom: 0.5rem;
		font-family: 'Great Vibes', 'Allura', cursive;
		font-size: clamp(1.8rem, 4vw, 2.5rem);
		font-weight: 400;
		color: #ffffff;
		opacity: 0;
		letter-spacing: 0.02em;
		text-shadow: 0 3px 20px rgba(0,0,0,0.6), 0 0 15px rgba(255,255,255,0.2);
		line-height: 1.2;
		position: relative;
		animation: subtitleFadeIn 1.2s ease-out 0.8s forwards;
		text-align: center;
	}
	
	.banner-text .subtitle::before {
		content: '';
		position: absolute;
		top: -10px;
		left: 50%;
		transform: translateX(-50%);
		width: 60px;
		height: 2px;
		background: linear-gradient(90deg, transparent, rgba(255,255,255,0.6), transparent);
		border-radius: 1px;
		opacity: 0;
		animation: subtitleLineFadeIn 0.8s ease-out 1.5s forwards;
	}
	.banner-text .venue {
		margin-top: 1rem;
		margin-bottom: 0.5rem;
		font-size: clamp(1.1rem, 2.5vw, 1.4rem);
		font-weight: 600;
		line-height: 1.4;
		text-align: center;
	}

	.banner-text .venue a {
		color: #ffffff;
		text-decoration: none;
		font-family: 'Playfair Display', serif;
		font-weight: 600;
		letter-spacing: 0.02em;
		text-shadow: 0 2px 8px rgba(0, 0, 0, 0.6);
		transition: all 0.3s ease;
		position: relative;
		display: inline-block;
		padding: 0.5rem 0.75rem;
		border-radius: 8px;
	}

	.banner-text .venue a:hover {
		color: #fce4ec;
		text-shadow: 0 0 20px rgba(252, 228, 236, 0.6);
		transform: translateY(-2px);
		background: rgba(252, 228, 236, 0.1);
	}

	.banner-text .venue a::after {
		content: ' 📍';
		font-size: 0.9em;
		opacity: 0.8;
		transition: opacity 0.3s ease;
	}

	.banner-text .venue a:hover::after {
		opacity: 1;
	}

	.top-date {
		position: absolute;
		top: 18px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 3;
		background: transparent;
		border: none;
		padding: 0;
		font-family: 'Playfair Display', serif;
		font-weight: 700;
		color: #fff;
		font-size: clamp(1.1rem, 3.2vw, 1.9rem);
		letter-spacing: 0.02em;
		text-shadow: 0 4px 18px rgba(0,0,0,0.6);
	}

	.countdown-container {
		position: absolute;
		top: 70px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 3;
		display: flex;
		gap: 1rem;
		align-items: center;
		justify-content: center;
	}

	.countdown-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		min-width: 60px;
	}

	.countdown-number {
		font-family: 'Playfair Display', serif;
		font-size: clamp(1.5rem, 3vw, 2.2rem);
		font-weight: 700;
		color: #ffffff;
		text-shadow: 0 2px 10px rgba(0, 0, 0, 0.8);
		line-height: 1;
		margin-bottom: 0.25rem;
	}

	.countdown-label {
		font-family: 'Inter', system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial;
		font-size: 0.75rem;
		font-weight: 500;
		color: rgba(255, 255, 255, 0.8);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		text-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
	}

	@media (max-width: 768px) {
		.countdown-container {
			top: 60px;
			gap: 0.75rem;
			padding: 0.5rem 1rem;
		}
		
		.countdown-item {
			min-width: 50px;
		}
		
		.countdown-number {
			font-size: clamp(1.2rem, 2.5vw, 1.8rem);
		}
		
		.countdown-label {
			font-size: 0.7rem;
		}
	}

	@media (max-width: 480px) {
		.countdown-container {
			flex-wrap: wrap;
			gap: 0.5rem;
			padding: 0.5rem;
		}
		
		.countdown-item {
			min-width: 45px;
		}
	}

	.content { 
		max-width: var(--page-max-width); 
		margin: 0 auto; 
		padding: 2rem; 
	}

	.title { font-size: 2.25rem; margin: 0; }
	.subtitle { margin: .25rem 0 1rem; color: #ddd; }
	.venue { margin: 0.25rem 0; color: #eee; }
	.rsvp {
		background: rgba(255, 255, 255, 0.2);
		color: #ffffff;
		border: 2px solid rgba(255, 255, 255, 0.4);
		padding: 0.875rem 2rem;
		border-radius: 25px;
		font-size: 1rem;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.3s ease;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		letter-spacing: 0.25px;
		text-transform: uppercase;
		font-family: 'Inter', sans-serif;
		min-width: 140px;
		backdrop-filter: blur(5px);
	}

	.rsvp:hover {
		background: rgba(255, 255, 255, 0.4);
		border-color: rgba(255, 255, 255, 0.6);
		color: #000000;
		transform: translateY(-2px);
		box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
	}

	.rsvp:active {
		transform: translateY(0);
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
	}

	.rsvp-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		opacity: 0;
		animation: fadeIn 0.4s ease-out forwards;
	}

	.rsvp-modal {
		background: linear-gradient(135deg, #ffffff 0%, #f8f9ff 100%);
		border-radius: 24px;
		padding: 0;
		max-width: 480px;
		width: 90%;
		max-height: 90vh;
		overflow-y: auto;
		box-shadow: 0 25px 60px rgba(0, 0, 0, 0.3), 0 0 0 1px rgba(255, 255, 255, 0.1);
		transform: scale(0.8) translateY(20px);
		opacity: 0;
		animation: modalSlideIn 0.5s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
		position: relative;
	}

	.rsvp-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 2rem 2rem 1.5rem;
		border-bottom: 1px solid rgba(0, 0, 0, 0.08);
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		border-radius: 24px 24px 0 0;
		color: white;
	}

	.rsvp-header h2 {
		margin: 0;
		font-size: 1.75rem;
		font-weight: 700;
		letter-spacing: -0.02em;
	}

	.close-button {
		background: rgba(255, 255, 255, 0.2);
		border: none;
		color: white;
		font-size: 1.5rem;
		width: 40px;
		height: 40px;
		border-radius: 50%;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.3s ease;
		font-weight: 300;
		line-height: 1;
	}

	.close-button:hover {
		background: rgba(255, 255, 255, 0.3);
		transform: scale(1.1);
	}

	.rsvp-modal form {
		padding: 2rem;
	}

	.form-group {
		margin-bottom: 1.75rem;
	}

	.label-text {
		display: block;
		margin-bottom: 0.5rem;
		color: #374151;
		font-weight: 600;
		font-size: 0.95rem;
		letter-spacing: 0.01em;
	}

	.form-group input,
	.form-group select {
		width: 100%;
		padding: 0.875rem 1rem;
		border: 2px solid rgba(0, 0, 0, 0.08);
		border-radius: 12px;
		font-size: 1rem;
		background: rgba(255, 255, 255, 0.9);
		transition: all 0.3s ease;
		font-family: inherit;
		box-sizing: border-box;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: var(--accent);
		background: #ffffff;
		box-shadow: 0 0 0 4px rgba(0, 83, 186, 0.1);
		transform: translateY(-1px);
	}

	.form-group input::placeholder {
		color: #9ca3af;
		font-style: italic;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 2rem;
		justify-content: flex-end;
	}

	.cancel-btn,
	.submit-btn {
		padding: 0.875rem 1.75rem;
		border-radius: 12px;
		font-weight: 600;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.3s ease;
		border: none;
		min-width: 120px;
	}

	.cancel-btn {
		background: rgba(107, 114, 128, 0.1);
		color: #6b7280;
		border: 2px solid rgba(107, 114, 128, 0.2);
	}

	.cancel-btn:hover {
		background: rgba(107, 114, 128, 0.2);
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(107, 114, 128, 0.15);
	}

	.submit-btn {
		background: linear-gradient(135deg, var(--accent) 0%, #004080 100%);
		color: white;
		box-shadow: 0 4px 15px rgba(0, 83, 186, 0.3);
	}

	.submit-btn:hover {
		background: linear-gradient(135deg, #004080 0%, #003060 100%);
		transform: translateY(-2px);
		box-shadow: 0 6px 20px rgba(0, 83, 186, 0.4);
	}

	.submit-btn:active,
	.cancel-btn:active {
		transform: translateY(0);
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes modalSlideIn {
		from {
			transform: scale(0.8) translateY(20px);
			opacity: 0;
		}
		to {
			transform: scale(1) translateY(0);
			opacity: 1;
		}
	}

	@keyframes subtitleFadeIn {
		from {
			opacity: 0;
			transform: translateY(10px);
		}
		to {
			opacity: 0.9;
			transform: translateY(0);
		}
	}

	@keyframes subtitleLineFadeIn {
		from {
			opacity: 0;
			width: 0;
		}
		to {
			opacity: 1;
			width: 60px;
		}
	}

	@media (max-width: 640px) {
		.rsvp-modal {
			width: 95%;
			margin: 1rem;
		}
		
		.rsvp-header {
			padding: 1.5rem 1.5rem 1rem;
		}
		
		.rsvp-modal form {
			padding: 1.5rem;
		}
		
		.actions {
			flex-direction: column;
		}
		
		.cancel-btn,
		.submit-btn {
			width: 100%;
		}
	}

	@media (max-width: 768px) {
		.banner-text .venue {
			margin-top: 0.75rem;
		}

		.banner-text .venue a {
			font-size: clamp(1rem, 2vw, 1.2rem);
			padding: 0.4rem 0.6rem;
		}
	}

	.radio-group {
		display: flex;
		gap: 1.5rem;
		align-items: center;
	}

	.radio-label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 1rem;
		font-weight: 500;
		color: #333;
		cursor: pointer;
	}

	.radio-label input {
		display: none;
	}

	.custom-radio {
		width: 20px;
		height: 20px;
		border: 2px solid #ccc;
		border-radius: 50%;
		display: inline-block;
		position: relative;
		transition: border-color 0.3s ease;
	}

	.radio-label input:checked + .custom-radio {
		border-color: #4caf50;
		background-color: #4caf50;
	}

	.radio-label input:checked + .custom-radio::after {
		content: "";
		width: 10px;
		height: 10px;
		background: white;
		border-radius: 50%;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

	textarea {
		width: 100%;
		max-width: 390px;
		resize: none;
		font-size: 1rem;
		padding: 0.75rem;
		border: 1px solid #ccc;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}
</style>
