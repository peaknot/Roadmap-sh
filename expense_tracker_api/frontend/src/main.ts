import './style.css'

interface Login {
  username: string,
  password: string,
}

interface RegisterUser {
  username: string,
  email: string,
  password: string,

}

const form = document.getElementById("user-registration") as HTMLFormElement;

form.addEventListener("submit", async (event) => {
    event.preventDefault()

    const username = (document.getElementById("username") as HTMLInputElement).value;
    const email = (document.getElementById("email") as HTMLInputElement).value;
    const password = (document.getElementById("password") as HTMLInputElement).value;

    const new_user: RegisterUser =  {
        username,
        email,
        password,
    }
    try {
    const response = await fetch("/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(new_user),
    });

    if (!response.ok) {
      throw new Error(`HTTP error: ${response.status}`);
    }

    const data = await response.json();
    console.log("User created:", data);
  } catch (err) {
    console.error("Registration failed:", err);
  }
});


const login_form = document.getElementById("login") as HTMLFormElement;
login_form.addEventListener("submit", async (event) => {
  event.preventDefault();

  const username = (document.getElementById("login-username") as HTMLInputElement).value;
  const password = (document.getElementById("login-password") as HTMLInputElement).value;

  const login: Login = {username, password};

  try {
    const response = await fetch("/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(login)
      });
      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }
      const data = await response.json();
      console.log("Login successful: ", data);
    }
    catch(err) {
    console.error("Login failed:", err)
  }

});
