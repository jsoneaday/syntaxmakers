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
  // an offset is equivalent to skip
  const [offset, setOffset] = useState(0);
  const [hasMorePreviousData, setHasMorePreviousData] = useState(false);
  const [hasMoreNextData, setHasMoreNextData] = useState(false);
  const [lastTriggerInit, setLastTriggerInit] = useState<string | undefined>();

  useEffect(() => {
    if (triggerInit && triggerInit != lastTriggerInit) {
      console.log("triggered");
      setLastTriggerInit(triggerInit);
      // since this is first load of data,
      // check if next load has any data
      dataQuery(PAGE_SIZE, false) //
        .then((moreData) => {
          if (moreData.length > 0) {
            setHasMoreNextData(true);
          } else {
            setHasMoreNextData(false);
          }
        })
        .catch((e) => console.log(e));
    }
  }, [triggerInit]);

  const getNextData = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    const nextOffset = setPagingDirection(PagingDirection.Next);
    await dataQuery(nextOffset, true);

    const moreDataOffset = getNextOffset(nextOffset, PagingDirection.Next);
    const moreData = await dataQuery(moreDataOffset, false);
    if (moreData.length > 0) {
      setHasMoreNextData(true);
    } else {
      setHasMoreNextData(false);
    }
    setHasMorePreviousData(true);
  };

  const getPreviousData = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    const nextOffset = setPagingDirection(PagingDirection.Previous);
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
  };

  const setPagingDirection = (newDirection: PagingDirection) => {
    let newOffset = getNextOffset(offset, newDirection);
    setOffset(newOffset);
    return newOffset;
  };

  return (
    <div className="paging-container">
      {hasMorePreviousData ? (
        <SecondaryButton onClick={getPreviousData}>previous</SecondaryButton>
      ) : (
        <div style={{ width: "50%" }}></div>
      )}
      {hasMoreNextData ? (
        <SecondaryButton onClick={getNextData}>next</SecondaryButton>
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
