function getGoogleOAuthURL() {
    const rootUrl : string = "https://accounts.google.com/o/oauth2/v2/auth";
  
    const option = {
      redirect_uri: "http://localhost:5173/api",
      client_id: "s", /*REPLACE WITH CLIENT ID */
      access_type: "offline",
      response_type: "code",
      prompt: "consent",
      scope: [
        "https://www.googleapis.com/auth/userinfo.profile",
        "https://www.googleapis.com/auth/userinfo.email",
        "openid"
      ].join(" "),
    };
  
    const qs = new URLSearchParams(option);
  
    return `${rootUrl}?${qs.toString()}`;
  }
  
  export default getGoogleOAuthURL;