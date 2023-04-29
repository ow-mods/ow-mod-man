import { useGetTranslation } from "@hooks";
import { SocketMessageType } from "@types";
import { memo, useCallback, useRef, useState } from "react";
import { LogFilter } from "./LogApp";
import { TranslationKey } from "@components/common/TranslationContext";

export interface LogHeaderProps {
    logsLen: number;
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
    onClear: () => void;
}

const LogHeader = memo(
    function LogHeader({ setActiveSearch, ...props }: LogHeaderProps) {
        const [tempSearch, setTempSearch] = useState<string>("");
        const searchTimeout = useRef<number | undefined>(undefined);
        const getTranslation = useGetTranslation();


        const onSearchChange = useCallback(
            (val: string) => {
                setTempSearch(val);
                if (searchTimeout) clearTimeout(searchTimeout.current);
                searchTimeout.current = setTimeout(() => {
                    setActiveSearch(val);
                }, 200);
            },
            [setActiveSearch]
        );

        return (
            <>
                <div className="log-actions">
                    <label htmlFor="search">
                        {getTranslation("SEARCH_LOGS")}
                        <input
                            type="text"
                            id="search"
                            value={tempSearch}
                            onChange={(e) => onSearchChange(e.target.value)}
                            placeholder="Search"
                        />
                    </label>
                    <label htmlFor="filter">
                        {getTranslation("FILTER")}
                        <select
                            id="filter"
                            value={props.activeFilter}
                            onChange={(e) => props.setActiveFilter(e.target.value as LogFilter)}
                        >
                            <>
                                <option value="Any">{getTranslation("ANY")}</option>
                                {Object.keys(SocketMessageType).map((k) => {
                                    {
                                        return (
                                            k !== "Fatal" &&
                                            k !== "Quit" && (
                                                <option key={k} value={k}>
                                                    {getTranslation(k as TranslationKey)}
                                                </option>
                                            )
                                        );
                                    }
                                })}
                            </>
                        </select>
                    </label>
                    <div>
                        <span>{getTranslation("LOG_COUNT", { count: props.logsLen.toString() })}</span>
                        <a
                            href={props.logsLen === 0 ? undefined : "#"}
                            role="button"
                            onClick={() => props.onClear()}
                        >
                            {getTranslation("CLEAR_LOGS")}
                        </a>
                    </div>
                </div>
            </>
        );
    },
    (current, next) =>
        current.activeFilter === next.activeFilter && current.logsLen === next.logsLen
);

export default LogHeader;
