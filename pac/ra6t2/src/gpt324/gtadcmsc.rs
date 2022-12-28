#[doc = "Register `GTADCMSC` reader"]
pub struct R(crate::R<GTADCMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTADCMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTADCMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTADCMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTADCMSC` writer"]
pub struct W(crate::W<GTADCMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTADCMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTADCMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTADCMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCMSC1` reader - A/D Conversion Start Request Compare Match Skipping Counter 1 Count Source Select"]
pub type ADCMSC1_R = crate::FieldReader<u8, ADCMSC1_A>;
#[doc = "A/D Conversion Start Request Compare Match Skipping Counter 1 Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCMSC1_A {
    #[doc = "0: Not counted (not skipped)"]
    _00 = 0,
    #[doc = "1: Counting GTADTRA register compare match"]
    _01 = 1,
    #[doc = "2: Counting GTADTRB register compare match"]
    _10 = 2,
    #[doc = "3: Counting both GTADTRA register compare match and GTADTRB register compare match"]
    _11 = 3,
}
impl From<ADCMSC1_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMSC1_A) -> Self {
        variant as _
    }
}
impl ADCMSC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMSC1_A {
        match self.bits {
            0 => ADCMSC1_A::_00,
            1 => ADCMSC1_A::_01,
            2 => ADCMSC1_A::_10,
            3 => ADCMSC1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADCMSC1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADCMSC1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADCMSC1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADCMSC1_A::_11
    }
}
#[doc = "Field `ADCMSC1` writer - A/D Conversion Start Request Compare Match Skipping Counter 1 Count Source Select"]
pub type ADCMSC1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTADCMSC_SPEC, u8, ADCMSC1_A, 2, O>;
impl<'a, const O: u8> ADCMSC1_W<'a, O> {
    #[doc = "Not counted (not skipped)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADCMSC1_A::_00)
    }
    #[doc = "Counting GTADTRA register compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADCMSC1_A::_01)
    }
    #[doc = "Counting GTADTRB register compare match"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADCMSC1_A::_10)
    }
    #[doc = "Counting both GTADTRA register compare match and GTADTRB register compare match"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADCMSC1_A::_11)
    }
}
#[doc = "Field `ADCMST1` reader - A/D Conversion Start Request Compare Match Skipping 1 Skipping Count Setting"]
pub type ADCMST1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMST1` writer - A/D Conversion Start Request Compare Match Skipping 1 Skipping Count Setting"]
pub type ADCMST1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCMSCNT1IV` reader - A/D Conversion Start Request Compare Match Skipping Counter 1 Initial Value"]
pub type ADCMSCNT1IV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMSCNT1IV` writer - A/D Conversion Start Request Compare Match Skipping Counter 1 Initial Value"]
pub type ADCMSCNT1IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCMSCNT1` reader - A/D Conversion Start Request Compare Match Skipping Counter 1"]
pub type ADCMSCNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMSC2` reader - A/D Conversion Start Request Compare Match Skipping Counter 2 Count Source Select"]
pub type ADCMSC2_R = crate::FieldReader<u8, ADCMSC2_A>;
#[doc = "A/D Conversion Start Request Compare Match Skipping Counter 2 Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCMSC2_A {
    #[doc = "0: Not counted (not skipped)"]
    _00 = 0,
    #[doc = "1: Counting GTADTRA register compare match"]
    _01 = 1,
    #[doc = "2: Counting GTADTRB register compare match"]
    _10 = 2,
    #[doc = "3: Counting both GTADTRA register compare match and GTADTRB register compare match"]
    _11 = 3,
}
impl From<ADCMSC2_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMSC2_A) -> Self {
        variant as _
    }
}
impl ADCMSC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCMSC2_A {
        match self.bits {
            0 => ADCMSC2_A::_00,
            1 => ADCMSC2_A::_01,
            2 => ADCMSC2_A::_10,
            3 => ADCMSC2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADCMSC2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADCMSC2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADCMSC2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADCMSC2_A::_11
    }
}
#[doc = "Field `ADCMSC2` writer - A/D Conversion Start Request Compare Match Skipping Counter 2 Count Source Select"]
pub type ADCMSC2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTADCMSC_SPEC, u8, ADCMSC2_A, 2, O>;
impl<'a, const O: u8> ADCMSC2_W<'a, O> {
    #[doc = "Not counted (not skipped)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADCMSC2_A::_00)
    }
    #[doc = "Counting GTADTRA register compare match"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADCMSC2_A::_01)
    }
    #[doc = "Counting GTADTRB register compare match"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADCMSC2_A::_10)
    }
    #[doc = "Counting both GTADTRA register compare match and GTADTRB register compare match"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADCMSC2_A::_11)
    }
}
#[doc = "Field `ADCMST2` reader - A/D Conversion Start Request Compare Match Skipping 2 Skipping Count Setting"]
pub type ADCMST2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMST2` writer - A/D Conversion Start Request Compare Match Skipping 2 Skipping Count Setting"]
pub type ADCMST2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCMSCNT2IV` reader - A/D Conversion Start Request Compare Match Skipping Counter 2 Initial Value"]
pub type ADCMSCNT2IV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCMSCNT2IV` writer - A/D Conversion Start Request Compare Match Skipping Counter 2 Initial Value"]
pub type ADCMSCNT2IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTADCMSC_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADCMSCNT2` reader - A/D Conversion Start Request Compare Match Skipping Counter 2"]
pub type ADCMSCNT2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - A/D Conversion Start Request Compare Match Skipping Counter 1 Count Source Select"]
    #[inline(always)]
    pub fn adcmsc1(&self) -> ADCMSC1_R {
        ADCMSC1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - A/D Conversion Start Request Compare Match Skipping 1 Skipping Count Setting"]
    #[inline(always)]
    pub fn adcmst1(&self) -> ADCMST1_R {
        ADCMST1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - A/D Conversion Start Request Compare Match Skipping Counter 1 Initial Value"]
    #[inline(always)]
    pub fn adcmscnt1iv(&self) -> ADCMSCNT1IV_R {
        ADCMSCNT1IV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - A/D Conversion Start Request Compare Match Skipping Counter 1"]
    #[inline(always)]
    pub fn adcmscnt1(&self) -> ADCMSCNT1_R {
        ADCMSCNT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - A/D Conversion Start Request Compare Match Skipping Counter 2 Count Source Select"]
    #[inline(always)]
    pub fn adcmsc2(&self) -> ADCMSC2_R {
        ADCMSC2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23 - A/D Conversion Start Request Compare Match Skipping 2 Skipping Count Setting"]
    #[inline(always)]
    pub fn adcmst2(&self) -> ADCMST2_R {
        ADCMST2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - A/D Conversion Start Request Compare Match Skipping Counter 2 Initial Value"]
    #[inline(always)]
    pub fn adcmscnt2iv(&self) -> ADCMSCNT2IV_R {
        ADCMSCNT2IV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - A/D Conversion Start Request Compare Match Skipping Counter 2"]
    #[inline(always)]
    pub fn adcmscnt2(&self) -> ADCMSCNT2_R {
        ADCMSCNT2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - A/D Conversion Start Request Compare Match Skipping Counter 1 Count Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmsc1(&mut self) -> ADCMSC1_W<0> {
        ADCMSC1_W::new(self)
    }
    #[doc = "Bits 4:7 - A/D Conversion Start Request Compare Match Skipping 1 Skipping Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn adcmst1(&mut self) -> ADCMST1_W<4> {
        ADCMST1_W::new(self)
    }
    #[doc = "Bits 8:11 - A/D Conversion Start Request Compare Match Skipping Counter 1 Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn adcmscnt1iv(&mut self) -> ADCMSCNT1IV_W<8> {
        ADCMSCNT1IV_W::new(self)
    }
    #[doc = "Bits 16:17 - A/D Conversion Start Request Compare Match Skipping Counter 2 Count Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn adcmsc2(&mut self) -> ADCMSC2_W<16> {
        ADCMSC2_W::new(self)
    }
    #[doc = "Bits 20:23 - A/D Conversion Start Request Compare Match Skipping 2 Skipping Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn adcmst2(&mut self) -> ADCMST2_W<20> {
        ADCMST2_W::new(self)
    }
    #[doc = "Bits 24:27 - A/D Conversion Start Request Compare Match Skipping Counter 2 Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn adcmscnt2iv(&mut self) -> ADCMSCNT2IV_W<24> {
        ADCMSCNT2IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer A/D Conversion Start Request Compare Match Skipping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtadcmsc](index.html) module"]
pub struct GTADCMSC_SPEC;
impl crate::RegisterSpec for GTADCMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtadcmsc::R](R) reader structure"]
impl crate::Readable for GTADCMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtadcmsc::W](W) writer structure"]
impl crate::Writable for GTADCMSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTADCMSC to value 0"]
impl crate::Resettable for GTADCMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
