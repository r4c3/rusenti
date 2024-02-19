<script lang="ts">
  const clientId: string = import.meta.env.VITE_CLIENT_ID;
  const redirectUri: string = import.meta.env.VITE_REDIRECT_URI;

  function loginWithGoogle(): void {
    const responseType: string = "code";
    const scope: string = "email profile openid";
    const authUrl: string = `https://accounts.google.com/o/oauth2/v2/auth?client_id=${clientId}&redirect_uri=${encodeURIComponent(
      redirectUri,
    )}&response_type=${responseType}&scope=${encodeURIComponent(
      scope,
    )}&prompt=consent`;

    window.location.href = authUrl;
  }
</script>

<main>
  <h1>skru</h1>
  <button on:click={loginWithGoogle}>Login with Google</button>
</main>
