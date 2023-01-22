import dayjs from 'dayjs';
import {
    Box,
    Flex,
    Grid,
    GridItem,
    HStack,
    Icon,
    IconButton,
    Text,
    Tooltip,
} from '@chakra-ui/react';
import getLoop from '~/common/utils/getLoop';
import React, { useState } from 'react';
import Icons from '~/common/helpers/Icons';
import { Habit, Target, TargetType } from '~/Habits/types';

const MonthlyCalendar = ({
    size,
    targets,
    habit,
    onCellClick,
}: {
    size?: 'sm' | 'md';
    targets: Target[];
    habit?: Habit;
    onCellClick?: (targetId: string | null, date: Date, newType: TargetType) => void;
}) => {
    if (!habit) {
        return null;
    }
    const targetsMap = targets.reduce((acc, target) => {
        acc[dayjs(target.date).format('DD/MM/YYYY')] = target;
        return acc;
    }, {} as { [key: string]: Target });
    size = size || 'md';
    const daysOfTheWeek = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
    const [monthId, setMonthId] = useState(dayjs().month());
    const [year, setYear] = useState(dayjs().year());

    const daysInMonth = dayjs(`${year}-${monthId + 1}-1`).daysInMonth();
    const date = dayjs(`${year}-${monthId + 1}-1`);
    const firstDay = date.day();
    const columns = Math.ceil((firstDay + daysInMonth) / 7);

    const sizeBinary = size === 'sm' ? 0 : 1;
    const gaps = [3, 12];
    const cellSizes = [12, 12];
    const gap = gaps[sizeBinary];
    const cellSize = cellSizes[sizeBinary];

    const handleSetMonth = (month: number) => {
        if (monthId === 12) {
            if (year <= 2023) {
                setYear(year + 1);
                setMonthId(0);
            }
        } else if (month === -1) {
            if (year > 2022) {
                setYear(year - 1);
                setMonthId(11);
            }
        } else {
            setMonthId(month);
        }
    };

    return (
        <Box p='2' textAlign={'center'}>
            <Flex justifyContent={'space-between'} alignItems={'center'} width={'100%'} pb={'2'}>
                <HStack spacing={'1'}>
                    <Tooltip label={'Previous year'} placement={'top'}>
                        <IconButton
                            aria-label='left'
                            icon={<Icon as={Icons.LeftDouble} />}
                            onClick={() => setYear(year - 1)}
                            disabled={year <= 2022}
                        />
                    </Tooltip>
                    <Tooltip label={'Previous month'} placement={'top'}>
                        <IconButton
                            aria-label='left'
                            icon={<Icon as={Icons.Left} />}
                            onClick={() => handleSetMonth(monthId - 1)}
                            disabled={year <= 2022 && monthId === 0}
                        />
                    </Tooltip>
                </HStack>
                <Text px={'2'}>{date.format('MMMM YYYY')}</Text>
                <HStack spacing={'1'}>
                    <Tooltip label={'Next month'} placement={'top'}>
                        <IconButton
                            aria-label='right'
                            icon={<Icon as={Icons.Right} />}
                            onClick={() => handleSetMonth(monthId + 1)}
                            disabled={year > 2023 && monthId === 11}
                        />
                    </Tooltip>

                    <Tooltip label={'Next year'} placement={'top'}>
                        <IconButton
                            aria-label='right'
                            icon={<Icon as={Icons.RightDouble} />}
                            onClick={() => setYear(year + 1)}
                            disabled={year > 2023}
                        />
                    </Tooltip>
                </HStack>
            </Flex>
            <Grid templateColumns={`repeat(7, ${cellSize + 30}px)`} gap={`${gap}px`}>
                {getLoop(7).map((rowId) => (
                    <GridItem key={'grid-column' + monthId + rowId}>
                        <Box>
                            <Box>
                                <Text py='2' textAlign='center' fontWeight='bold'>
                                    {daysOfTheWeek[rowId]}
                                </Text>
                            </Box>

                            <Grid templateRows={`repeat(${columns}, 1fr)`} gap={`${gap}px`}>
                                {getLoop(columns).map((columnId) => (
                                    <Cell
                                        year={year}
                                        key={'cell' + monthId + columnId + rowId}
                                        columnId={columnId}
                                        rowId={rowId}
                                        rawMonthId={monthId}
                                        rawDayId={columnId * 7 + rowId - firstDay}
                                        daysInMonth={daysInMonth}
                                        size={cellSize}
                                        targetsMap={targetsMap}
                                        habit={habit}
                                        onCellClick={onCellClick}
                                        setMonthId={setMonthId}
                                    />
                                ))}
                            </Grid>
                        </Box>
                    </GridItem>
                ))}
            </Grid>
        </Box>
    );
};

const Cell = ({
    rawDayId,
    rawMonthId,
    columnId,
    rowId,
    year,
    daysInMonth,
    size,
    targetsMap,
    habit,
    onCellClick,
    setMonthId,
}: {
    rawDayId: number;
    rawMonthId: number;
    columnId: number;
    rowId: number;
    year: number;
    daysInMonth: number;
    size: number;
    targetsMap: Record<string, Target>;
    habit: Habit;
    setMonthId: (month: number) => void;
    onCellClick?: (targetId: string | null, date: Date, newType: TargetType) => void;
}) => {
    const sizePx = `${size}px`;

    const handleClick = () => {
        if (monthId !== rawMonthId) {
            setMonthId(monthId);
            return;
        }
        let newType;

        if (!target) {
            newType = TargetType.Done;
        } else if (target.targetType === TargetType.Done && habit?.allowSkip) {
            newType = TargetType.Skip;
        } else {
            newType = TargetType.Empty;
        }

        onCellClick?.(target?.id, day.toDate(), newType);
    };

    function getMonthId(day: number) {
        let month;
        if (day > daysInMonth - 1) {
            month = rawMonthId + 1;
        } else if (day < 0) {
            month = rawMonthId - 1;
        } else {
            month = rawMonthId;
        }

        if (month > 11) {
            month = 0;
        }
        if (month < 0) {
            month = 11;
        }
        return month;
    }

    function getPrevMonthDate() {
        let y = year;
        let m = monthId;
        if (m < 0) {
            m = 11;
            y = year - 1;
        } else if (m > 11) {
            m = 0;
            y = year + 1;
        }

        return dayjs(`${y}-${m + 1}-1`);
    }

    function getDayId(columnId: number, rowId: number, firstDay: number) {
        let day = columnId * 7 + rowId - firstDay;
        if (day > daysInMonth - 1) {
            return day - daysInMonth;
        } else if (day >= 0) {
            return day;
        } else {
            const prevDate = getPrevMonthDate();
            return prevDate.daysInMonth() + day;
        }
    }

    const monthId = getMonthId(rawDayId);
    const firstDay = dayjs(`${year}-${rawMonthId + 1}-1`).day();
    const dayId = getDayId(columnId, rowId, firstDay);
    const day = dayjs(`${year}-${monthId + 1}-${dayId + 1}`);

    const target = targetsMap[day.format('DD/MM/YYYY')];
    return (
        <Box cursor='pointer'>
            <Tooltip
                label={
                    dayjs(`2023-${monthId + 1}-${dayId + 1}`).format('D MMMM YYYY') +
                    `${target && target?.targetType === TargetType.Skip ? ' (skip)' : ''}`
                }
            >
                {target && target.targetType === TargetType.Skip ? (
                    <Box
                        p={2}
                        borderRadius='50%'
                        color={'black'}
                        bg={'green.100'}
                        onClick={handleClick}
                        _hover={{
                            bg: 'green.200',
                        }}
                        transition={'all 0.2s'}
                    >
                        {day.format('D')}
                    </Box>
                ) : (
                    <Box
                        p={2}
                        width={sizePx + 20}
                        height={sizePx + 20}
                        borderRadius='50%'
                        color={
                            target ? 'white' : monthId !== rawMonthId ? 'blackAlpha.600' : 'black'
                        }
                        bg={target ? 'green.500' : 'transparent'}
                        onClick={handleClick}
                        _hover={{
                            bg: target ? 'green.600' : 'gray.200',
                        }}
                        transition={'all 0.2s'}
                    >
                        {day.format('D')}
                    </Box>
                )}
            </Tooltip>
        </Box>
    );
};

export default MonthlyCalendar;
