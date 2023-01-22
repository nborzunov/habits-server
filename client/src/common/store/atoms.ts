import { atom, selector } from 'recoil';

import { Layout } from 'react-grid-layout';
import { Habit } from '~/Habits/types';
import { User } from '~/Profile/types';

export const habitsState = atom<Habit[]>({
    key: 'habitsState',
    default: [],
});

export const uncompletedHabitsState = selector({
    key: 'uncompletedHabitsState',
    get: ({ get }) => {
        const list = get(habitsState);

        return list.filter((habit) => !habit.completedToday);
    },
});

export const completedHabitsState = selector({
    key: 'completedHabitsState',
    get: ({ get }) => {
        const list = get(habitsState);

        return list.filter((habit) => habit.completedToday);
    },
});

const localStorageEffect =
    (key: string) =>
    ({ setSelf, onSet }: { setSelf: any; onSet: any }) => {
        const savedValue = localStorage.getItem(key);
        if (savedValue != null) {
            setSelf(JSON.parse(savedValue));
        }

        onSet((newValue: string, _: any, isReset: boolean) => {
            isReset
                ? localStorage.removeItem(key)
                : localStorage.setItem(key, JSON.stringify(newValue));
        });
    };

export const selectedHabitIdState = atom<string | null>({
    key: 'selectedHabitIdState',
    default: null,
    effects: [localStorageEffect('selectedHabit')],
});

export const selectedHabitState = selector({
    key: 'selectedHabitState',
    get: ({ get }) => {
        const list = get(habitsState);

        const selectedId = get(selectedHabitIdState);
        return list.find((habit) => habit.id == selectedId);
    },
});

export const activeUserState = atom<User | null>({
    key: 'activeUserState',
    default: null,
});

export const tokenState = atom<String | null>({
    key: 'tokenState',
    default: null,
    effects: [localStorageEffect('authToken')],
});

// export const widgetsState = atom<String | null>({
//     key: 'tokenState',
//     default: [],
//     effects: [localStorageEffect('widgets')],
// });

export const layoutState = atom<Layout[] | undefined>({
    key: 'layoutState',
    default: undefined,
    // effects: [localStorageEffect('layout')],
});
