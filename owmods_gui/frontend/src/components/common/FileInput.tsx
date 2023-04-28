import { useTranslation } from "@hooks";
import { dialog } from "@tauri-apps/api";
import { IconType } from "react-icons";
import { BsFolderFill } from "react-icons/bs";
import Icon from "./Icon";

export interface FileInputProps<T> {
    dialogOptions: T;
    id: string;
    label?: string;
    className?: string;
    browseButtonIcon?: IconType;
    value?: string;
    onChange?: (path: string) => void;
    tooltip?: string;
    tooltipPlacement?: string;
}

const FileInput = <T,>(openFunc: (options?: T) => Promise<string | string[] | null>) =>
    function FileInput(props: FileInputProps<T>) {
        const browseLabel = useTranslation("BROWSE");
        const label = useTranslation(props.id);

        const onBrowse = () => {
            openFunc(props.dialogOptions).then((path) => {
                if (path !== null) {
                    props.onChange?.(path as string);
                }
            });
        };

        return (
            <div className={props.className}>
                <label
                    data-tooltip={props.tooltip}
                    data-placement={props.tooltipPlacement ?? "bottom"}
                    htmlFor={props.id}
                >
                    {props.label ?? label}
                </label>
                <div className="file-input-row">
                    <input
                        id={props.id}
                        name="target"
                        value={props.value ?? ""}
                        onChange={(e) => props.onChange?.(e.target.value)}
                    />
                    <button onClick={onBrowse} className="browse-button fix-icons" type="button">
                        <Icon iconType={props.browseButtonIcon ?? BsFolderFill} /> {browseLabel}
                    </button>
                </div>
            </div>
        );
    };

export const OpenFileInput = FileInput(dialog.open);
export const SaveFileInput = FileInput(dialog.save);
