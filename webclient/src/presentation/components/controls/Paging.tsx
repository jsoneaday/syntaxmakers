import { MouseEvent, useEffect, useState } from "react";
import { SecondaryButton } from "./Buttons";
import { PAGE_SIZE } from "../../common/Paging";
import "../../theme/paging.css";

enum PagingDirection {
  Next,
  Previous,
}

export type DataQuery<T> = (
  nextOffset: number,
  setData: boolean
) => Promise<T[]>;

interface PagingProps<T> {
  triggerInit: string | undefined;
  /// this function should run your data call
  dataQuery: DataQuery<T>;
}

export function Paging<T>({ triggerInit, dataQuery }: PagingProps<T>) {
  const [offset, setOffset] = useState(0);
  const [hasMorePreviousData, setHasMorePreviousData] = useState(false);
  const [hasMoreNextData, setHasMoreNextData] = useState(false);
  const [priorDisabled, setPriorDisabled] = useState(false);
  const [nextDisabled, setNextDisabled] = useState(false);

  useEffect(() => {
    if (triggerInit) {
      console.log("init");
      getNextData(true);
    }
  }, [triggerInit]);

  const onGetNextData = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    await getNextData(false);
  };

  const getNextData = async (init: boolean) => {
    setNextDisabled(true);
    let nextOffset = offset;
    if (!init) {
      nextOffset = setOffsetByDirection(offset, PagingDirection.Next);
    }
    await dataQuery(nextOffset, true);

    const moreDataOffset = getNextOffset(nextOffset, PagingDirection.Next);
    const moreData = await dataQuery(moreDataOffset, false);
    if (moreData.length > 0) {
      setHasMoreNextData(true);
    } else {
      setHasMoreNextData(false);
    }

    if (init) {
      setHasMorePreviousData(false);
    } else {
      setHasMorePreviousData(true);
    }
    setNextDisabled(false);
  };

  const onGetPreviousData = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    let nextOffset = setOffsetByDirection(offset, PagingDirection.Previous);
    await dataQuery(nextOffset, true);

    if (nextOffset === 0) {
      setHasMorePreviousData(false);
      setHasMoreNextData(true);
    } else {
      const moreDataOffset = getNextOffset(
        nextOffset,
        PagingDirection.Previous
      );
      const moreData = await dataQuery(moreDataOffset, false);
      if (moreData.length > 0) {
        setHasMorePreviousData(true);
      } else {
        setHasMorePreviousData(false);
      }
      setHasMoreNextData(true);
    }
    setPriorDisabled(false);
  };

  const setOffsetByDirection = (
    offset: number,
    newDirection: PagingDirection
  ) => {
    let newOffset = getNextOffset(offset, newDirection);
    setOffset(newOffset);
    return newOffset;
  };

  return (
    <div className="paging-container">
      {hasMorePreviousData ? (
        <SecondaryButton onClick={onGetPreviousData} disabled={priorDisabled}>
          previous
        </SecondaryButton>
      ) : (
        <div style={{ width: "50%" }}></div>
      )}
      {hasMoreNextData ? (
        <SecondaryButton onClick={onGetNextData} disabled={nextDisabled}>
          next
        </SecondaryButton>
      ) : null}
    </div>
  );
}

const getNextOffset = (offset: number, newDirection: PagingDirection) => {
  if (newDirection === PagingDirection.Next) {
    return offset + PAGE_SIZE;
  }
  return offset - PAGE_SIZE <= 0 ? 0 : offset - PAGE_SIZE;
};
