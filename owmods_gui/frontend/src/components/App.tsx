import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import { invoke } from "@tauri-apps/api";
import { TranslationContext } from "./TranslationContext";

// Refresh once to get data
invoke("refresh_local_db").catch(() => console.warn("Can't fetch local DB"));
invoke("refresh_remote_db").catch(() => console.warn("Can't fetch remote DB"));

const App = () => {
    return (
        <TranslationContext.Provider value="English">
            <main className="container">
                <header>
                    <Nav />
                </header>
                <Tabs />
            </main>
        </TranslationContext.Provider>
    );
};

export default App;
