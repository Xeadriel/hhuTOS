use alloc::{boxed::Box, string::String};

use crate::{devices::{cga, lfb::{self, get_lfb, BLACK, TRUE_WHITE, WHITE}}, library::input::{get_last_ch, getch}, user::the_lost_sword::{dungeon_cg, forest_cg, game_over, game_over_sword, sound_effects::{play, play_no_thread, SoundEffect}, title_screen, title_sword, title_sword_small, village_cg}};
// use crate::{devices::{cga, lfb::{self, get_lfb, BLACK, TRUE_WHITE, WHITE}}, library::input::{get_last_ch, getch}, user::the_lost_sword::{dungeon_cg, game_over, game_over_sword, sound_effects::{play, play_no_thread, SoundEffect}, title_screen, title_sword, title_sword_small, village_cg}};

pub struct Story {
    player_name: Option<Box<str>>,
}

impl Story {
    pub const fn new() -> Self {
        Self {
            player_name: None,
        }
    }

    pub fn set_player_name(&mut self, name: &str) {
        self.player_name = Some(Box::from(name));
    }

    pub fn get_player_name(&self) -> Option<&str> {
        self.player_name.as_deref()
    }

    pub fn play_title_screen(&mut self) {
        {
            play(SoundEffect::MainTheme);
            let mut lfb = get_lfb().lock();
        
            lfb.clear();

            lfb.draw_bitmap_slowly(341, 140, title_sword::WIDTH, title_sword::HEIGHT, title_sword::DATA, 1); // small: lfb.draw_bitmap(360, 200, title_sword::WIDTH, title_sword::HEIGHT, title_sword::DATA);
            
            lfb.draw_bitmap_slowly(195, 320, title_screen::WIDTH, title_screen::HEIGHT, title_screen::DATA, 1);
            lfb.draw_str(311, 400, WHITE, "Press Any Key to Start");
            getch();
        }
    }

    pub fn play_intro(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        lfb.draw_str(300, 100, WHITE, "Enter your name:");

        let mut name = String::from("Xeadriel");
        loop {
            let c:char = get_last_ch();
            if c == ' ' {
                lfb.draw_str(300, 120, WHITE, &name);
                continue;
            }

            if c == '\n' {
                break;
            }

            if c == '\x08' {
                lfb.draw_str(300, 120, BLACK, &name);
                name.pop();
                lfb.draw_str(300, 120, WHITE, &name);
                continue;
            }
            lfb.draw_str(300, 120, BLACK, &name);
            name.push(c);
            lfb.draw_str(300, 120, WHITE, &name);
        }

        if name == "" {
            name = String::from("Xeadriel");
        }
        self.set_player_name(&name);
        lfb.clear();

        // 'Come on, then,' I spat, tightening my grip on the sword. The torch in my other hand flickered, casting wild shadows on the cave walls. 'You want me? Come get me.'
        lfb.draw_str_slowly(100, 50, TRUE_WHITE, "'Come on, then,' I spat, tightening my grip on the sword.", 20);
        lfb.draw_str_slowly(100, 65, TRUE_WHITE,"The torch in my other hand flickered, casting wild shadows on the cave walls.", 20); 
        lfb.draw_str_slowly(100, 80, TRUE_WHITE,"'You want me? Come get me.'", 20);
        getch();

        // The monster growled low, crouched on all fours like some nightmare-wolf, muscles twitching beneath patchy fur. Its eyes locked on mine. It charged.
        lfb.draw_str_slowly(100, 100, TRUE_WHITE,"The monster growled low, crouched on all fours like some nightmare-wolf,", 20);
        lfb.draw_str_slowly(100, 115, TRUE_WHITE,"muscles twitching beneath patchy fur. Its eyes locked on mine. It charged.", 20);
        getch();

        // 'Shit-' I barely dodged, the thing's claws skimming past my face. I slashed at its side, felt the blade bite, heard it scream.
        lfb.draw_str_slowly(100, 130, TRUE_WHITE,"'Shit-' I barely dodged, the thing's claws skimming past my face.", 20);
        lfb.draw_str_slowly(100, 145, TRUE_WHITE,"I slashed at its side, felt the blade bite, heard it scream.", 20);
        getch();

        // 'That got your attention, huh?' I panted. My shoulder throbbed, slick with blood.
        lfb.draw_str_slowly(100, 160, TRUE_WHITE,"'That got your attention, huh?' I panted. My shoulder throbbed,", 20);
        lfb.draw_str_slowly(100, 175, TRUE_WHITE,"slick with blood.", 20);
        getch();

        // It turned on me again. I braced.
        lfb.draw_str_slowly(100, 190, TRUE_WHITE,"It turned on me again. I braced.", 20);
        getch();

        // 'Let's finish this.'
        lfb.draw_str_slowly(300, 230, TRUE_WHITE,"'Let's finish this.'", 20);
        getch();

        // I stepped in, blade raised-but the ground trembled.
        lfb.draw_str_slowly(100, 250, TRUE_WHITE,"I stepped in, blade raised-but the ground trembled.", 20);
        getch();
        
        // 'What now?' I muttered. Then the floor collapsed beneath us.
        lfb.draw_str_slowly(100, 265, TRUE_WHITE,"'What now?' I muttered. Then the floor collapsed beneath us.", 20);
        getch();
        
        // There wasn't time to scream. Just falling. Darkness. Light. A sound like tearing silk.
        lfb.draw_str_slowly(100, 280, TRUE_WHITE,"There wasn't time to scream. Just falling. Darkness. Light. A sound like tearing silk.", 20);
        getch();

        // Then-thud. I hit something soft. Grass?
        lfb.draw_str_slowly(100, 300, TRUE_WHITE,"Then... *thud*. I hit something soft. Grass?", 20);
        getch();

        // I groaned, rolled over. 'Where the hell-'
        lfb.draw_str_slowly(100, 315, TRUE_WHITE,"I groaned, rolled over. 'Where the hell-'", 20);
        getch();

        // Above me: sky. Real sky. Trees. Birds that didn't sound right. The cave was gone.
        lfb.draw_str_slowly(100, 330, TRUE_WHITE,"Above me: sky. Real sky. Trees. Birds that didn't sound right. The cave was gone.", 20);
        getch();

        lfb.draw_bitmap(0, 0, forest_cg::WIDTH, forest_cg::HEIGHT, forest_cg::DATA);
        lfb.flush();
        getch();
        lfb.clear();

        let mut y = 50;
        // I sat up fast-and cursed. My main sword was nowhere in sight. Must've slipped during the fall.
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"I sat up fast-and cursed. My main sword was nowhere in sight.", 20);
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"Must've slipped during the fall.", 20);
        getch();

        // 'Perfect.' I reached to my side and pulled the short blade from its sheath. 'Good thing I brought a spare.'
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"'Perfect.' I reached to my side and pulled the short blade from its sheath.", 20);
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"'Good thing I brought a spare.'", 20);
        getch();

        // A low rumble echoed from deeper in the forest. Not the monster.
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"A low rumble echoed from deeper in the forest. Not the monster.", 20);
        getch();

        // 'Oh, fantastic,' I muttered, dragging myself to my feet. 'Portal beast first, now mystery forest threat. Great day.'
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"'Oh, fantastic,' I muttered, dragging myself to my feet.", 20);
        y += 15;
        lfb.draw_str_slowly(100, y, TRUE_WHITE,"'Portal beast first, now mystery forest threat. Great day.'", 20);
        getch();

        // I didn't lower my sword.
        y += 50;
        lfb.draw_str_slowly(300, y, TRUE_WHITE,"I raised my sword.", 150);
        getch();
    }

    pub fn play_village_story(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        let x = 100;
        let mut y = 50;
        let color = TRUE_WHITE;
        // I'd been walking for what felt like hours.
        lfb.draw_str_slowly(x, y, color, "I'd been walking for what felt like hours.", 20);
        getch();

        // The bloodied short sword felt heavier with every step. My stomach growled. My legs ached.

        y += 15;
        lfb.draw_str_slowly(x, y, color, "The bloodied short sword felt heavier with every step.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "My stomach growled. My legs ached.", 20);
        getch();

        // Then, over a hill: rooftops. Voices. A village.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Then, over a hill: Rooftops. Voices. A village.", 20);
        getch();

        lfb.draw_bitmap(0, 0, village_cg::WIDTH, village_cg::HEIGHT, village_cg::DATA);
        lfb.flush();
        getch();

        lfb.clear();

        // I didn't run, but I didn't take my time either.
        y = 50;
        lfb.draw_str_slowly(x, y, color, "I didn't run, but I didn't take my time either.", 20);
        getch();

        // The people stared as I entered, some wary, others curious.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "The people stared as I entered, some wary, others curious.", 20);
        getch();

        // That's when a young guy, about my age, but with the face of innonce of a kid, came 
        // bounding up to me like I was some returning war hero.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "That's when a young guy, about my age, but with the face of innonce of a kid, came ", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "bounding up to me like I was some returning war hero.", 20);
        getch();

        // 'Whoa! Are you from the rift?' he asked, eyes wide. 'You are, aren't you?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Whoa! Are you from the rift?' he asked, eyes wide.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You are, aren't you?'", 20);
        getch();

        // 'Maybe,' I said cautiously. 'Who's asking?'
        // 'I'm Yaen. Come on, you have to meet Elder Toni.
        // He'll want to talk to you.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Maybe,' I said cautiously. 'Who's asking?'", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'I'm Yaen. Come on, you have to meet Elder Toni.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He'll want to talk to you.'", 20);
        getch();

        // Before I could argue, he was already dragging me through the village.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Before I could argue, he was already dragging me through the village.", 20);


        // The elder's house was set apart on a small rise.
        // Quiet. Simple. Inside, the air smelled of incense and old wood.
        y += 25;
        lfb.draw_str_slowly(x, y, color, "The elder's house was set apart on a small rise.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Quiet. Simple. Inside, the air smelled of incense and old wood.", 20);
        getch();

        // Toni was waiting, seated cross-legged, ancient and calm.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni was waiting, seated cross-legged, meditating.", 20);
        getch();

        // His eyes were sharp though, too sharp. He looked at me like he already knew.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "His eyes were sharp though, too sharp.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He looked at me like he already knew.", 20);
        getch();

        // 'So,' he said softly. 'You came through the rift.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'So,' he said softly. 'You came through the rift.'", 20);
        getch();

        // 'You mean the portal in the cave floor? Yeah.
        // Didn't exactly have a choice.' He gave a slow nod.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You mean the portal in the cave floor? Yeah.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Didn't exactly have a choice.'", 20);
        getch();
        
        // He gave a slow nod. 'Then it is true. The prophecy... it has begun.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He gave a slow nod. 'Then it is true. The prophecy... it has begun.'", 20);
        getch();

        // I raised an eyebrow. 'You want to maybe explain what you're talking about?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I raised an eyebrow. 'You want to maybe explain what you're talking about?'", 20);
        getch();

        // Toni leaned forward. 'Long ago, a demon called Drag-Tul nearly destroyed our world.
        // Only one stood a chance against him, an ancient hero, Elenion. 
        // But even he could not kill Drag-Tul.
        // Instead, he sealed the demon into his own sword, binding it with blood and will.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni leaned forward. 'Long ago, a demon called Drag-Tul nearly destroyed our world.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Only one stood a chance against him, an ancient hero, Elenion.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "But even he could not kill Drag-Tul. Instead,' he continued,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'he sealed the demon into its own sword, binding it with blood and will.'", 20);
        getch();

        // 'And let me guess,' I said, arms crossed, 'that seal's about to break?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'And let me guess,' I said, arms crossed, 'that seal's about to break?'", 20);
        getch();

        // 'Yes,' Toni said ignoring my snarky undertone. 'And the legend speaks of another,
        // one who would appear from nowhere in the hour of need. 
        // A stranger not born of this land, but tied to its fate.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Yes,' Toni said ignoring my snarky undertone. 'And the legend speaks of another,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "one who would appear from nowhere in the hour of need. A stranger not born of this land,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "but tied to its fate.'", 20);
        getch();

        // 'You think that's me?' Toni didn't blink. 'I know it is.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You think that's me?' Toni didn't blink.", 20);
        getch();
        y += 25;
        lfb.draw_str_slowly(x+200, y, color, "'I know it is.'", 50);
        getch();

        lfb.clear();
        y = 50; 
        // I stared at him for a long moment. 'Look… all I want is a way back to my world.' 
        lfb.draw_str_slowly(x, y, color, "I stared at him for a long moment. 'Look... all I want is a way back to my world.'", 20);
        getch();

        // Toni just stared back, reminding me of my old man, that stubborn bastard.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni just stared back, reminding me of my old man, that stubborn bastard.", 20);
        getch();

        // 'But I suppose if helping you gets me that... fine. I'll do it.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'But I suppose if helping you gets me that... fine. I'll do it.'", 20);
        getch();
        
        // 'I'll help too!' Yaen piped up from the doorway. 'You won't have to do it alone!'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'I'll help too!' Yaen piped up from the doorway. 'You won't have to do it alone!'", 20);
        getch();

        // Toni smiled faintly. 'Then you must both begin at once. 
        // However... there is a technique Elenion once used: Sael'varan, the Flaming Fist of Light. 
        // It's the only force said to be able to end Drag-Tul's existence completely. 
        // I can teach it to you, but it demands focus. Discipline.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni smiled faintly. 'Then you must both begin at once. However...", 20);
        y += 15;
        lfb. draw_str_slowly(x, y, color, "there is a technique Elenion once used:", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Sael'varan, the Flaming Fist of Light.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I can teach it to you, but it demands focus. Discipline.'", 20);
        getch();

        // I just looked at the old man, then at Yaen, who was practically vibrating with excitement.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I just looked at the old man, then at Yaen,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "who was practically vibrating with excitement.", 20);
        getch();

        // I sighed, adjusting my grip on the short sword.
        // 'Alright. Let's get started.' 
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I sighed, adjusting my grip on the short sword.", 20);
        y += 25;
        lfb.draw_str_slowly(x+200, y, color, "'Alright. Let's get started.'", 50);
        getch();
    }

    pub fn play_game_over(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();
        
        lfb.draw_bitmap_slowly(331, 140, game_over_sword::WIDTH, game_over_sword::HEIGHT, game_over_sword::DATA, 1);
        lfb.draw_bitmap_slowly(196, 320, game_over::WIDTH, game_over::HEIGHT, game_over::DATA, 1);
        lfb.draw_str(228, 430, WHITE, "Press Any Key To Return To The Title Screen");
        play_no_thread(SoundEffect::GameOver);
        getch();
    }
    
    pub fn play_flame_fist_training1(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        let x = 100;
        let mut y = 50;
        let color = TRUE_WHITE;


        // Months passed. The village no longer stared when I walked through. 
        // The fields, the trees, the air,I'd stopped looking at them like they didn't belong to me. 
        // I sparred daily, bled often, and trained until my body forgot the old world's weight.
        lfb.draw_str_slowly(x, y, color, "Months passed. The village no longer stared when I walked through.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "The fields, the trees, the air... They almost became something like a home.", 20);
        getch();

        // Toni stopped calling me 'hero' after the first week. Now it was just Xeadriel.
        y += 15;
        let msg = String::from("Toni stopped calling me 'hero' after the first week. Now it was just ") + self.get_player_name().unwrap() + ".";
        lfb.draw_str_slowly(x, y, color, &msg, 20);
        getch();

        // Yaen and I had grown closer too - he was still loud, still quick to speak, but we'd learned to move together. 
        // He covered my blind spots in training and always had some ridiculous comment to throw at my worst moments. I guess we made a decent team.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Yaen and I had grown closer too - he was still loud,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "still quick to speak, but we'd learned to move together.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He covered my blind spots in training and always had some", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "ridiculous comment to throw at my worst moments.", 20);
        y += 25;
        lfb.draw_str_slowly(300, y, color, "I guess we made a decent team.", 20);
        getch();

        // One cool morning, Toni stood before me at the edge of the training field, 
        // arms folded behind his back, the rising sun catching the edges of his robes.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "One cool morning, Toni stood before me at the edge of the training field,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "arms folded behind his back, the rising sun catching the edges of his robes.", 20);
        getch();

        // 'You're ready,' he said. No ceremony. Just quiet certainty.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You're ready,' he said. No ceremony. Just quiet certainty.", 20);
        getch();

        // I wiped the sweat from my brow. 'You sure about that?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I wiped the sweat from my brow. 'You sure about that?'", 20);
        getch();

        // Toni gave me one of those calm, infuriating smiles. 'Try it. Let the fire come from focus, not rage.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni gave me one of those calm, infuriating smiles.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Try it. Let the fire come from focus, not rage.'", 20);
        getch();

        // I took a breath. Closed my eyes. Felt it - the current I'd been chasing for weeks,
        // running beneath skin and bone like heat waiting for release.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I took a breath. Closed my eyes. Felt it - the current I'd been chasing for weeks,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "running beneath skin and bone like heat waiting for release.", 20);
        getch();
    }

    
    pub fn play_flame_fist_training2(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        let x = 100;
        let mut y = 50;
        let color = TRUE_WHITE;

        // Fire exploded across my arm, bright and clean, not wild like before. The strike cracked the stone target in half.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Fire exploded across my arm, bright and clean, not wild like before.", 20);
        getch();

        // I stared at it. My arm smoked, but didn't burn. 
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I stared at it. My arm smoked, but didn't burn.", 20);
        getch();
        
        // Despite not using my entire strength which would otherwise knock me out,
        // I could feel it, though, that hollowed-out ache deep in my chest.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Despite not using my entire strength, which would otherwise knock me out,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I could feel it, that hollowed-out ache deep in my chest.", 20);
        getch();

        // Toni nodded. 'You're ready to go. But remember, you'll only get one shot.
        // The Sael'varan takes too much from you. Miss, and you won't get another.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Toni nodded. 'You're ready to go. But remember, you'll only get one shot.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "The Sael'varan takes too much from you. Miss, and you won't get another.'", 20);
        getch();

        // As if the world had been waiting for him to say it, the ground trembled 
        // beneath us, deep and low, like the earth itself growled.
        y += 25;
        lfb.draw_str_slowly(x, y, color, "As if the world had been waiting for him to say it, the ground trembled", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "beneath us, deep and low, like the earth itself growled.", 20);
        getch();

        // Yaen ran up from the village path, pale and wide-eyed. 'It's started, hasn't it?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Yaen ran up from the village path, pale and wide-eyed.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'It's started, hasn't it?'", 20);
        getch();

        // 'Before you go,' Toni went to grab something, 'I wanted to give you this', he said.
        // 'The villagers found it. It washed up by the river near the old pine grove. I had it cleaned.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Before you go,' Toni went to grab something, 'I wanted to give you this.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "The villagers found it. It washed up by the river near the old pine grove.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I had it cleaned.'", 20);
        getch();

        y += 15;
        lfb.draw_bitmap_slowly(350, y, title_sword_small::WIDTH, title_sword_small::HEIGHT, title_sword_small::DATA, 5);
        getch();

        // 'My sword! I thought it would be lost forever.', I said, 'You have my thanks.'
        y += 150;
        lfb.draw_str_slowly(x, y, color, "'My lost sword! I thought it would be gone forever.', I said, 'You have my thanks.'", 20);
        getch();

        // He handed it to me, sheathed in worn leather. The weight was familiar, after all this time.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He handed it to me, sheathed in worn leather. The weight was familiar,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "after all this time.", 20);
        getch();

        // I looked to Yaen. 'Let's move.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I looked to Yaen. 'Let's move.'", 20);
        getch();

        // We didn't look back.
        y += 25;
        lfb.draw_str_slowly(330, y, color, "We didn't look back.", 100);
        getch();

        // [You Can Now Press F To Perform A Close Combat Fire Attack. Use It To Defeat Drag-Tul When He Is On His Last Heart.]
        y += 25;
        lfb.draw_str_slowly(x+60, y, color, "[You Can Now Press F To Perform A Close Combat Fire Attack.]", 20);
        y += 15;
        lfb.draw_str_slowly(x+70, y, color, "[Use It To Defeat Drag-Tul When He Is On His Last Heart.]", 20);
        y += 15;
        lfb.draw_str_slowly(280, y, color, "[Missing Will Result In Death.]", 20);
        getch();

        lfb.draw_bitmap(0, 0, dungeon_cg::WIDTH, dungeon_cg::HEIGHT, dungeon_cg::DATA);
        lfb.flush();
        getch();
    }

    pub fn play_boss_story(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        let x = 100;
        let mut y = 50;
        let color = TRUE_WHITE;
        
        lfb.draw_bitmap_slowly(350, y, title_sword::WIDTH, title_sword::HEIGHT, title_sword::DATA, 5);

        y += title_sword::HEIGHT + 50;
        lfb.draw_str_slowly(x, y, color, "'This must be it. This must be the sword of Drag-Tul.', Yaen said. 'What now?'", 20);
        getch();
        
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'Now, we -', I didn't finish my sentence as the sword started to shake.", 20);
        getch();
        
        y += 15;
        lfb.draw_str_slowly(x, y, color, "It leaked... smoke? It formed... a body, a face.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You... all of you. You will pay for this humiliation you have brought upon me.'", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "His voice was deep and distorted.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color,"'My name shall be last thing you hear before you die.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I shall put you down first and bring the rest of your filthy kin down with you.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "For I am Drag-Tul, the Sword of Death.'", 20);
        getch();

        y += 15;
        lfb.draw_str_slowly(x, y, color, "We gave each other one last determined nod.", 20);
        getch();

        y += 50;
        lfb.draw_str_slowly(330, y, color, "We were ready.", 100);
        getch();


    }
    
    pub fn play_victory(&mut self) {
        let mut lfb = get_lfb().lock();
        lfb.clear();

        let x = 100;
        let mut y = 50;
        let color = TRUE_WHITE;

        // Drag-Tul towered over the scorched ruins of the room, 
        //a shadow wrapped in old fire and armor etched with faces that still screamed.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Drag-Tul towered over the scorched ruins of the room,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "a shadow wrapped in old fire and armor etched with faces that still screamed.", 20);
        getch();

        // I charged.
        y += 25;
        lfb.draw_str_slowly(350, y, color, "I charged.", 100);
        getch();

        // Steel met steel, again and again. I struck his shoulder, his ribs, his throat,
        //nothing slowed him. He swung back, and I ducked, rolled, came up slicing.
        y += 25;
        lfb.draw_str_slowly(x, y, color, "Steel met steel, again and again. I struck his shoulder, his ribs, his throat,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "nothing slowed him. He swung back, and I ducked, rolled, came up slicing.", 20);
        getch();

        // His fist grazed my side, pain shot up my ribs.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "His fist grazed my side, pain shot up my ribs.", 20);
        getch();

        // Yaen's arrows sang past me, thudding into Drag-Tul's chest
        // like nails into stone. 'He's weakening!' Yaen shouted.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Yaen's arrows sang past me, thudding into Drag-Tul's chest", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "like nails into stone. 'He's weakening!' Yaen shouted.", 20);
        getch();

        // But he wasn't.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "But he wasn't.", 20);
        getch();

        // I spun and cut down two of the shrieking minions that broke from the mist.
        // One clawed my arm before I slit its throat. 
        // Another leapt, Yaen's arrow took it mid-air.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I spun and cut down two of the shrieking minions that broke from the mist.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "One clawed my arm before I slit its throat.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Another leapt, Yaen's arrow took it mid-air.", 20);
        getch();

        // I pressed in, blade flashing. Drag-Tul growled,
        //swinging wide. I slipped under the arc and stabbed deep. Nothing. No blood. No wound.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I pressed in, blade flashing. Drag-Tul growled, swinging wide.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I slipped under the arc and stabbed deep. Nothing. No blood. No wound.", 20);
        getch();

        // My sword bounced off like I'd hit a wall. I staggered back, panting.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "My sword bounced off like I'd hit a wall. I staggered back, panting.", 20);
        getch();

        // 'He's not taking damage!' I shouted.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'He's not taking damage!' I shouted.", 20);
        getch();

        // 'That's it!' Yaen called. 'Use it! The Sael'varan! Now!'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'That's it!' Yaen called. 'Use it! The Sael'varan! Now!'", 20);
        getch();

        // I looked down at my arm. It was shaking. I hadn't used it since the field. This was it. One chance.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I looked down at my arm. It was shaking. I had fought through several rooms after all.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "This was it. One chance.", 20);
        getch();

        // I dropped the sword. Took a breath.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I dropped the sword. Took a breath.", 20);
        getch();

        // I stepped forward as Drag-Tul raised his hand for the killing blow. My fist blazed white-hot.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I stepped forward as Drag-Tul raised his hand for the killing blow.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "My fist blazed white-hot.", 20);
        getch();

        // I drove it into his chest.
        y += 25;
        lfb.draw_str_slowly(330, y, color, "I drove it into his chest.", 100);
        getch();

        // Light erupted. Fire screamed.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Light erupted. Fire screamed.", 20);
        getch();

        // Drag-Tul roared once, then crumbled into ash and shadow, 
        // pulled inward by a force older than death.
        // 'You... how... did... you...'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Drag-Tul roared once, then crumbled into ash and shadow,", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "pulled inward by a force older than death.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You... how... did... you...'", 100);
        getch();

        // Silence fell.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Silence fell.", 20);
        getch();

        // I dropped to my knees. 'It's done.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I dropped to my knees. 'It's done.'", 20);
        getch();

        // Yaen ran over, laughing through tears, pulling me upright. 'We did it.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Yaen ran over, laughing through tears, pulling me upright.", 20);
        getch();
        y += 25;
        lfb.draw_str_slowly(330, y, color, "'We did it.'", 100);
        getch();

        lfb.clear();

        y = 50;

        // I stayed in the village for a few more days. 
        // They treated us like legends. It was... nice. Toni gave me a rare smile. 
        // He nodded like he'd always known this was how it would end.
        lfb.draw_str_slowly(x, y, color, "I stayed in the village for a few more days. They treated us like legends.", 20);
        y += 15;
        getch();
        lfb.draw_str_slowly(x, y, color, "It was... nice.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color,"Toni gave me a rare smile.", 20);
        getch();

        // But it wasn't the end. Not really.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "But it wasn't the end. Not really.", 20);
        getch();

        // The portal shimmered back into existence,same cave, same light.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "The portal shimmered back into existence, it showed the same cave, same light.", 20);
        getch();

        // I stood at the edge, sword at my back, Yaen at my side.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I stood at the edge, sword at my back, Yaen at my side.", 20);
        getch();

        // 'You coming?' I asked.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You coming?' I asked.", 20);
        getch();

        // He hesitated, staring back at the village. 'You're serious?'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "He hesitated, staring back at the village.", 20);
        y += 15;
        lfb.draw_str_slowly(x, y, color, "'You're serious?'", 20);
        getch();

        // I smiled. 'You'd hate going back to normal life. 
        // Besides… I could use the backup.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "I smiled. 'You'd hate going back to normal life.", 20);
        getch();
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Besides... I could use the backup.'", 20);
        getch();

        // Yaen grinned. 'You'll never get rid of me now.'
        y += 15;
        lfb.draw_str_slowly(x, y, color, "Yaen grinned. 'You'll never get rid of me now.'", 20);
        getch();

        // We stepped through back to my world, together.
        y += 15;
        lfb.draw_str_slowly(x, y, color, "We stepped through the portal back to my world, together.", 20);
        getch();

        // The next adventure waited. 
        y += 25;
        lfb.draw_str_slowly(330, y, color, "The next adventure waited.", 100);
        getch();
    }
}
