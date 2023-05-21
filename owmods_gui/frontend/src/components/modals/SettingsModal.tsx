import {
    ChangeEvent,
    ReactNode,
    forwardRef,
    memo,
    useCallback,
    useEffect,
    useImperativeHandle,
    useRef,
    useState
} from "react";
import { Config, GuiConfig, Language, OWMLConfig, Theme } from "@types";
import Modal from "./Modal";
import { useGetTranslation } from "@hooks";
import { commands, hooks } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import Icon from "@components/common/Icon";
import { BsArrowCounterclockwise } from "react-icons/bs";
import { type TranslationKey, TranslationNameMap } from "@components/common/TranslationContext";
import { os } from "@tauri-apps/api";

const ThemeArr = Object.values(Theme);
const LanguageArr = Object.values(Language);

interface SettingsFormProps {
    initialConfig: Config;
    initialOwmlConfig: OWMLConfig;
    initialGuiConfig: GuiConfig;
}

interface SettingsFormHandle {
    save: () => void;
}

interface SettingsRowProps {
    label: string;
    id: string;
    children?: ReactNode;
    tooltip?: string;
    tooltipPlacement?: string;
}

interface SettingsTextProps extends SettingsRowProps {
    value: string;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

interface SettingsSwitchProps extends SettingsRowProps {
    value: boolean;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

interface SettingsSelectProps extends SettingsRowProps {
    value: string;
    options: readonly string[];
    translate: boolean;
    onChange?: (e: ChangeEvent<HTMLSelectElement>) => void;
    nameMap?: Record<string, string>;
}

const SettingsRow = (props: SettingsRowProps) => {
    return (
        <div className="settings-row">
            <label
                data-tooltip={props.tooltip}
                data-placement={props.tooltipPlacement ?? "bottom"}
                htmlFor={props.id}
            >
                {props.label}
            </label>
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

const SettingsSelect = (props: SettingsSelectProps) => {
    const getTranslation = useGetTranslation();

    return (
        <SettingsRow {...props}>
            <select value={props.value} id={props.id} onChange={(e) => props.onChange?.(e)}>
                {props.options.map((o) => (
                    <option key={o} value={o}>
                        {props.translate
                            ? getTranslation(o as TranslationKey)
                            : props.nameMap?.[o] ?? o}
                    </option>
                ))}
            </select>
        </SettingsRow>
    );
};

const SettingsFolder = (props: SettingsTextProps) => {
    const getTranslation = useGetTranslation();

    const onChange = (e: string) => {
        props.onChange?.({
            target: {
                id: props.id,
                value: e
            }
        } as ChangeEvent<HTMLInputElement>);
    };

    return (
        <OpenFileInput
            id={props.id}
            label={props.label}
            value={props.value}
            onChange={onChange}
            className="settings-row"
            dialogOptions={{
                defaultPath: props.value,
                directory: true,
                multiple: false,
                title: getTranslation("SELECT", { name: props.label })
            }}
            tooltip={props.tooltip}
            tooltipPlacement={props.tooltipPlacement}
        />
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
            <label data-tooltip={props.tooltip} data-placement="right" htmlFor={props.id}>
                {props.label}
            </label>
        </div>
    );
};

const ResetButton = memo(function ResetButton(props: { onClick: () => void }) {
    const getTranslation = useGetTranslation();

    return (
        <a
            className="reset-button"
            aria-label={getTranslation("RESET")}
            data-placement="left"
            data-tooltip={getTranslation("RESET")}
            onClick={props.onClick}
            href="#"
        >
            <Icon iconType={BsArrowCounterclockwise} />
        </a>
    );
});

const SettingsForm = forwardRef(function SettingsForm(props: SettingsFormProps, ref) {
    const [config, setConfig] = useState<Config>(props.initialConfig);
    const [owmlConfig, setOwmlConfig] = useState<OWMLConfig>(props.initialOwmlConfig);
    const [guiConfig, setGuiConfig] = useState<GuiConfig>(props.initialGuiConfig);
    const getTranslation = useGetTranslation();

    const [showLogServerOption, setShowLogServerOption] = useState<boolean>(false);

    useEffect(() => {
        os.platform().then((p) => {
            setShowLogServerOption(p === "win32");
        });
    }, []);

    useImperativeHandle(
        ref,
        () => ({
            save: () => {
                const task = async () => {
                    await commands.saveConfig({ config });
                    await commands.saveGuiConfig({ guiConfig });
                    if (config.owmlPath !== props.initialConfig.owmlPath) {
                        await commands.refreshLocalDb();
                    } else {
                        await commands.saveOwmlConfig({ owmlConfig });
                    }
                    if (config.databaseUrl !== props.initialConfig.databaseUrl) {
                        await commands.refreshRemoteDb();
                    }
                };
                task().catch(console.error);
            }
        }),
        [config, owmlConfig, guiConfig, props.initialConfig]
    );

    const getVal = (e: HTMLInputElement | HTMLSelectElement) => {
        const type = e.type;
        if (type && type === "checkbox") {
            return (e as HTMLInputElement).checked;
        } else {
            return e.value;
        }
    };

    const handleConf = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setConfig({ ...config, [e.target.id]: getVal(e.target) });
    };

    const handleOwml = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setOwmlConfig({ ...owmlConfig, [e.target.id]: getVal(e.target) });
    };

    const handleGui = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setGuiConfig({ ...guiConfig, [e.target.id]: getVal(e.target) });
    };

    const onReset = useCallback((i: number) => {
        commands.getDefaultConfigs().then((data) => {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            [setConfig, setGuiConfig, setOwmlConfig][i](data[i]);
        });
    }, []);

    return (
        <form className="settings">
            <h4>
                {getTranslation("GUI_SETTINGS")} <ResetButton onClick={() => onReset(1)} />
            </h4>
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.theme}
                translate
                label={getTranslation("THEME")}
                options={ThemeArr}
                id="theme"
            />
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.language}
                translate={false}
                label={getTranslation("LANGUAGE")}
                options={LanguageArr}
                id="language"
                nameMap={TranslationNameMap}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.rainbow}
                label={getTranslation("RAINBOW")}
                id="rainbow"
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.watchFs}
                label={getTranslation("WATCH_FS")}
                id="watchFs"
                tooltip={getTranslation("TOOLTIP_WATCH_FS")}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.noWarning}
                label={getTranslation("DISABLE_WARNING")}
                id="noWarning"
                tooltip={getTranslation("TOOLTIP_DISABLE_WARNING")}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.logMultiWindow}
                label={getTranslation("LOG_MULTI_WINDOW")}
                id="logMultiWindow"
                tooltip={getTranslation("TOOLTIP_LOG_MULTI_WINDOW")}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.autoEnableDeps}
                label={getTranslation("AUTO_ENABLE_DEPS")}
                id="autoEnableDeps"
                tooltip={getTranslation("TOOLTIP_AUTO_ENABLE_DEPS")}
            />
            {showLogServerOption && (
                <SettingsSwitch
                    onChange={handleGui}
                    value={guiConfig.noLogServer}
                    label={getTranslation("LET_OWML_HANDLE_LOGS")}
                    id="noLogServer"
                    tooltip={getTranslation("TOOLTIP_LET_OWML_HANDLE_LOGS")}
                />
            )}
            <h4>
                {getTranslation("OWML_SETTINGS")} <ResetButton onClick={() => onReset(2)} />
            </h4>
            <SettingsFolder
                onChange={handleOwml}
                value={owmlConfig.gamePath}
                label={getTranslation("GAME_PATH")}
                id="gamePath"
                tooltip={getTranslation("TOOLTIP_GAME_PATH")}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.forceExe}
                label={getTranslation("FORCE_EXE")}
                id="forceExe"
                tooltip={getTranslation("TOOLTIP_FORCE_EXE")}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.debugMode}
                label={getTranslation("DEBUG_MODE")}
                id="debugMode"
                tooltip={getTranslation("TOOLTIP_OWML_DEBUG_MODE")}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.incrementalGC}
                label={getTranslation("INCREMENTAL_GC")}
                id="incrementalGC"
                tooltip={getTranslation("TOOLTIP_INCREMENTAL_GC")}
            />
            <h4>
                {getTranslation("GENERAL_SETTINGS")} <ResetButton onClick={() => onReset(0)} />
            </h4>
            <SettingsText
                onChange={handleConf}
                value={config.databaseUrl}
                label={getTranslation("DB_URL")}
                id="databaseUrl"
                tooltip={getTranslation("TOOLTIP_DATABASE_URL")}
            />
            <SettingsText
                onChange={handleConf}
                value={config.alertUrl}
                label={getTranslation("ALERT_URL")}
                id="alertUrl"
                tooltip={getTranslation("TOOLTIP_ALERT_URL")}
            />
            <SettingsFolder
                onChange={handleConf}
                value={config.owmlPath}
                label={getTranslation("OWML_PATH")}
                id="owmlPath"
                tooltip={getTranslation("TOOLTIP_OWML_PATH")}
                tooltipPlacement="top"
            />
        </form>
    );
});

const SettingsModal = forwardRef(function SettingsModal(_: object, ref) {
    const settingsFormRef = useRef<SettingsFormHandle>();

    const [configStatus, config, err1] = hooks.getConfig("CONFIG_RELOAD");
    const [guiConfigStatus, guiConfig, err2] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");
    const [owmlConfigStatus, owmlConfig, err3] = hooks.getOwmlConfig("OWML_CONFIG_RELOAD");

    const status = [configStatus, guiConfigStatus, owmlConfigStatus];

    const getTranslation = useGetTranslation();

    if (status.includes("Loading")) {
        return <></>;
    } else if (status.includes("Error")) {
        return (
            <Modal
                showCancel
                heading={getTranslation("SETTINGS")}
                confirmText={getTranslation("SAVE")}
                ref={ref}
            >
                <>
                    <p className="center">
                        <>
                            Error: Couldn&apos;t Load Settings: {err1 ?? ""} {err2 ?? ""}{" "}
                            {err3 ?? ""}
                        </>
                    </p>
                </>
            </Modal>
        );
    } else {
        return (
            <Modal
                onConfirm={() => settingsFormRef.current?.save()}
                showCancel
                heading={getTranslation("SETTINGS")}
                confirmText={getTranslation("SAVE")}
                ref={ref}
            >
                <SettingsForm
                    ref={settingsFormRef}
                    initialConfig={config!}
                    initialGuiConfig={guiConfig!}
                    initialOwmlConfig={owmlConfig!}
                />
            </Modal>
        );
    }
});

export default SettingsModal;
