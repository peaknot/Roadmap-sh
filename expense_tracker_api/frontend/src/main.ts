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

interface NewExpense {
  expense_desc: string,
  amount: number,
  category: string,
}
interface Expense {
  id: number,
  expense_desc: string,
  amount: number,
  category: string,
  created_at: string,
}

interface ExpenseList {
  Expenses: Expense[],
}

const form = document.getElementById("user-registration");
if (form instanceof HTMLFormElement) {
  form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const username = (document.getElementById("username") as HTMLInputElement).value;
    const email = (document.getElementById("email") as HTMLInputElement).value;
    const password = (document.getElementById("password") as HTMLInputElement).value;

    const new_user: RegisterUser = {
      username,
      email,
      password,
    };
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
}

const login_form = document.getElementById("login");
if (login_form instanceof HTMLFormElement) {
  login_form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const username = (document.getElementById("login-username") as HTMLInputElement).value;
    const password = (document.getElementById("login-password") as HTMLInputElement).value;

    const login: Login = { username, password };

    try {
      const response = await fetch("/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(login),
      });

      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }
      const data = await response.json() as { token?: string};
      if (!data.token ) throw new Error("Login response missing token");

      localStorage.setItem("token", data.token);

      window.location.href = "home.html";
      console.log("Login successful: ", data);
      
      if (document.getElementById("new-expense")) {
        void loadExpenses();
      }

    } catch (err) {
      console.error("Login failed:", err);
    }
  });
}

const expense_form = document.getElementById("new-expense");
if (expense_form instanceof HTMLFormElement) {
  expense_form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const description = (document.getElementById("description") as HTMLInputElement).value;
    const amount = Number((document.getElementById("amount") as HTMLInputElement).value);
    if (Number.isNaN(amount)) {
      console.error("Amount must be a valid number");
      return;
    }
    const category = (document.getElementById("category") as HTMLSelectElement).value.toLowerCase();

    const new_expense: NewExpense = {
      expense_desc: description,
      amount,
      category,
    };
    const token = localStorage.getItem("token");
    if (!token) {
      console.error("No auth token found. Please login again.");
      return;
    }

    try {
      const response = await fetch("/home/expense/add", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${token}`,
        },
        body: JSON.stringify(new_expense),
      });

      if (response.status === 401) {
        localStorage.removeItem("token");
        window.location.href = "index.html";
        return;
      }

      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }

      const data = await response.json();
      console.log("New expense added", data);
      await loadExpenses();
    } catch (err) {
      console.log("Failed to add expense", err);
    }
  });
}

const expense_list = document.getElementById("expense-list");

function renderExpenses(expenses: Expense[]) {
  if (!(expense_list instanceof HTMLElement)) {
    return;
  }
  expense_list.innerHTML = "";

  if (!expenses || expenses.length === 0) {
    expense_list.textContent = "No expenses yet.";
    return;
  }

  for (const exp of expenses) {
    const li= document.createElement("li")
    li.textContent = `${exp.expense_desc} | ${exp.amount} | ${exp.category} | ${new Date(exp.created_at).toLocaleString()}`;
    expense_list.appendChild(li);
  }
}

async function loadExpenses(search?: string) {
  const token = localStorage.getItem("token");
  if (!token) {
    window.location.href = "index.html";
    return;
  }

  const url = new URL("/home/expense/list", window.location.origin);
  if (search?.trim()) url.searchParams.set("search", search.trim());

  const response = await fetch(url.toString(), {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });

  if (response.status === 401) {
    localStorage.removeItem("token");
    window.location.href = "index.html";
    return;
  }

  if (!response.ok) {
    throw new Error(`HTTP error: ${response.status}`);
  }

  const data = (await response.json()) as ExpenseList;
  renderExpenses(data.Expenses ?? []);
}

const searchBtn = document.getElementById("expense-search-btn");
if (searchBtn instanceof HTMLButtonElement) {
  searchBtn.addEventListener("click", () => {
    const q = (document.getElementById("expense-search") as HTMLInputElement)?.value ?? "";
    void loadExpenses(q);
  });
}
