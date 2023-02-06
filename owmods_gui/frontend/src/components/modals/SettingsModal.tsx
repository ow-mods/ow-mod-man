import Icon from "@components/Icon";
import { ChangeEvent, ReactNode, useState } from "react";
import { FaFolder } from "react-icons/fa";
import { Config, defaultConfig, defaultOwmlConfig, OwmlConfig } from "@types";
import Modal, { ModalWrapperProps } from "./Modal";

interface SettingsRowProps {
    label: string;
    id: string;
    children?: ReactNode;
}

interface SettingsTextProps extends SettingsRowProps {
    value: string;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

interface SettingsSwitchProps extends SettingsRowProps {
    value: boolean;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

const SettingsRow = (props: SettingsRowProps) => {
    return (
        <div className="settings-row">
            <label htmlFor={props.id}>{props.label}</label>
            <div>{props.children}</div>
        </div>
    );
};

const SettingsText = (props: SettingsTextProps) => {
    return (
        <SettingsRow {...props}>
            <input
                onChange={(e) => props.onChange?.(e)}
                name={props.id}
                value={props.value}
                id={props.id}
            />
            {props.children}
        </SettingsRow>
    );
};

const SettingsFolder = (props: SettingsTextProps) => {
    return (
        <SettingsText {...props}>
            <button className="fix-icons" type="button">
                <Icon iconType={FaFolder} /> Browse
            </button>
        </SettingsText>
    );
};

const SettingsSwitch = (props: SettingsSwitchProps) => {
    return (
        <div className="settings-row fix-icons">
            <input
                onChange={(e) => props.onChange?.(e)}
                type="checkbox"
                role="switch"
                checked={props.value}
                id={props.id}
                name={props.id}
            />
            <label htmlFor={props.id}>{props.label}</label>
        </div>
    );
};

const SettingsModal = (props: ModalWrapperProps) => {
    const [config, setConfig] = useState<Config>(defaultConfig);
    const [owmlConfig, setOwmlConfig] = useState<OwmlConfig>(defaultOwmlConfig);

    const handleConf = (e: ChangeEvent<HTMLInputElement>) => {
        setConfig({ ...config, [e.target.id]: e.target.value });
    };

    const handleOwml = (e: ChangeEvent<HTMLInputElement>) => {
        setOwmlConfig({ ...owmlConfig, [e.target.id]: e.target.value });
    };

    return (
        <Modal showCancel heading="Settings" confirmText="Save" open={props.open}>
            <form className="settings">
                <h4>General Settings</h4>
                <SettingsText
                    onChange={handleConf}
                    value={config.databaseUrl}
                    label="Database URL"
                    id="databaseUrl"
                />
                <SettingsText
                    onChange={handleConf}
                    value={config.alertUrl}
                    label="Alert URL"
                    id="alertUrl"
                />
                <SettingsFolder
                    onChange={handleConf}
                    value={config.owmlPath}
                    label="OWML Path"
                    id="owmlPath"
                />
                <SettingsFolder
                    onChange={handleConf}
                    value={config.winePrefix}
                    label="Wine Prefix"
                    id="winePrefix"
                />
                <h4>OWML Settings</h4>
                <SettingsFolder
                    onChange={handleOwml}
                    value={owmlConfig.gamePath}
                    label="Game Path"
                    id="gamePath"
                />
                <SettingsSwitch
                    onChange={handleOwml}
                    value={owmlConfig.forceExe}
                    label="Force Exe"
                    id="forceExe"
                />
                <SettingsSwitch
                    onChange={handleOwml}
                    value={owmlConfig.debugMode}
                    label="Debug Mode"
                    id="debugMode"
                />
                <SettingsSwitch
                    onChange={handleOwml}
                    value={owmlConfig.incrementalGC}
                    label="Incremental GC"
                    id="incrementalGC"
                />
            </form>
        </Modal>
    );
};

export default SettingsModal;
