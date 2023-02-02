import Nav from "@components/nav/Nav";
import Tabs from "@components/tabs/Tabs";
import LocalMods from "@components/mods/local/LocalMods";

export default () => {
    return (
        <main className="container">
            <header>
                <Nav />
                <Tabs />
            </header>
            <hr />
            <LocalMods />
        </main>
    );
};
