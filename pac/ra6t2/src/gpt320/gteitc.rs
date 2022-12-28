#[doc = "Register `GTEITC` reader"]
pub struct R(crate::R<GTEITC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTEITC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTEITC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTEITC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTEITC` writer"]
pub struct W(crate::W<GTEITC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTEITC_SPEC>;
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
impl From<crate::W<GTEITC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTEITC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIVTC1` reader - Extended Interrupt Skipping Counter 1 Count Source Select"]
pub type EIVTC1_R = crate::FieldReader<u8, EIVTC1_A>;
#[doc = "Extended Interrupt Skipping Counter 1 Count Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EIVTC1_A {
    #[doc = "0: Not counted (not skipped)"]
    _00 = 0,
    #[doc = "1: Counting both at overflow or underflow in saw-wave mode, and counting crests in triangle-wave mode or complementary PWM mode"]
    _01 = 1,
    #[doc = "2: Counting both at overflow or underflow in saw-wave mode, and counting troughs in triangle-wave mode or complementary PWM mode"]
    _10 = 2,
    #[doc = "3: Counting both at overflow or underflow in saw-wave mode, and counting both crests and troughs in triangle-wave mode or complementary PWM mode"]
    _11 = 3,
}
impl From<EIVTC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EIVTC1_A) -> Self {
        variant as _
    }
}
impl EIVTC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIVTC1_A {
        match self.bits {
            0 => EIVTC1_A::_00,
            1 => EIVTC1_A::_01,
            2 => EIVTC1_A::_10,
            3 => EIVTC1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EIVTC1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EIVTC1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EIVTC1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EIVTC1_A::_11
    }
}
#[doc = "Field `EIVTC1` writer - Extended Interrupt Skipping Counter 1 Count Source Select"]
pub type EIVTC1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTEITC_SPEC, u8, EIVTC1_A, 2, O>;
impl<'a, const O: u8> EIVTC1_W<'a, O> {
    #[doc = "Not counted (not skipped)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EIVTC1_A::_00)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting crests in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EIVTC1_A::_01)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting troughs in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EIVTC1_A::_10)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting both crests and troughs in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EIVTC1_A::_11)
    }
}
#[doc = "Field `EIVTT1` reader - Extended Interrupt Skipping 1 Skipping Count Setting"]
pub type EIVTT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIVTT1` writer - Extended Interrupt Skipping 1 Skipping Count Setting"]
pub type EIVTT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITC_SPEC, u8, u8, 4, O>;
#[doc = "Field `EITCNT1` reader - Extended Interrupt Skipping Counter 1"]
pub type EITCNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIVTC2` reader - Extended Interrupt Skipping Counter 2 Count Source select"]
pub type EIVTC2_R = crate::FieldReader<u8, EIVTC2_A>;
#[doc = "Extended Interrupt Skipping Counter 2 Count Source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EIVTC2_A {
    #[doc = "0: Not counted (not skipped)"]
    _00 = 0,
    #[doc = "1: Counting both at overflow or underflow in saw-wave mode, and counting crests in triangle-wave mode or complementary PWM mode"]
    _01 = 1,
    #[doc = "2: Counting both at overflow or underflow in saw-wave mode, and counting troughs in triangle-wave mode or complementary PWM mode"]
    _10 = 2,
    #[doc = "3: Counting both at overflow or underflow in saw-wave mode, and counting both crests and troughs in triangle-wave mode or complementary PWM mode"]
    _11 = 3,
}
impl From<EIVTC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EIVTC2_A) -> Self {
        variant as _
    }
}
impl EIVTC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIVTC2_A {
        match self.bits {
            0 => EIVTC2_A::_00,
            1 => EIVTC2_A::_01,
            2 => EIVTC2_A::_10,
            3 => EIVTC2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EIVTC2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EIVTC2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == EIVTC2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == EIVTC2_A::_11
    }
}
#[doc = "Field `EIVTC2` writer - Extended Interrupt Skipping Counter 2 Count Source select"]
pub type EIVTC2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GTEITC_SPEC, u8, EIVTC2_A, 2, O>;
impl<'a, const O: u8> EIVTC2_W<'a, O> {
    #[doc = "Not counted (not skipped)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(EIVTC2_A::_00)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting crests in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(EIVTC2_A::_01)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting troughs in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(EIVTC2_A::_10)
    }
    #[doc = "Counting both at overflow or underflow in saw-wave mode, and counting both crests and troughs in triangle-wave mode or complementary PWM mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(EIVTC2_A::_11)
    }
}
#[doc = "Field `EIVTT2` reader - Extended Interrupt Skipping 2 Skipping Count Setting"]
pub type EIVTT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EIVTT2` writer - Extended Interrupt Skipping 2 Skipping Count Setting"]
pub type EIVTT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITC_SPEC, u8, u8, 4, O>;
#[doc = "Field `EITCNT2IV` reader - Extended Interrupt Skipping Counter 2 Initial Value"]
pub type EITCNT2IV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EITCNT2IV` writer - Extended Interrupt Skipping Counter 2 Initial Value"]
pub type EITCNT2IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTEITC_SPEC, u8, u8, 4, O>;
#[doc = "Field `EITCNT2` reader - Extended Interrupt Skipping Counter 2"]
pub type EITCNT2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Extended Interrupt Skipping Counter 1 Count Source Select"]
    #[inline(always)]
    pub fn eivtc1(&self) -> EIVTC1_R {
        EIVTC1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Extended Interrupt Skipping 1 Skipping Count Setting"]
    #[inline(always)]
    pub fn eivtt1(&self) -> EIVTT1_R {
        EIVTT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Extended Interrupt Skipping Counter 1"]
    #[inline(always)]
    pub fn eitcnt1(&self) -> EITCNT1_R {
        EITCNT1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Extended Interrupt Skipping Counter 2 Count Source select"]
    #[inline(always)]
    pub fn eivtc2(&self) -> EIVTC2_R {
        EIVTC2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Extended Interrupt Skipping 2 Skipping Count Setting"]
    #[inline(always)]
    pub fn eivtt2(&self) -> EIVTT2_R {
        EIVTT2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Extended Interrupt Skipping Counter 2 Initial Value"]
    #[inline(always)]
    pub fn eitcnt2iv(&self) -> EITCNT2IV_R {
        EITCNT2IV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Extended Interrupt Skipping Counter 2"]
    #[inline(always)]
    pub fn eitcnt2(&self) -> EITCNT2_R {
        EITCNT2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Extended Interrupt Skipping Counter 1 Count Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn eivtc1(&mut self) -> EIVTC1_W<0> {
        EIVTC1_W::new(self)
    }
    #[doc = "Bits 4:7 - Extended Interrupt Skipping 1 Skipping Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn eivtt1(&mut self) -> EIVTT1_W<4> {
        EIVTT1_W::new(self)
    }
    #[doc = "Bits 16:17 - Extended Interrupt Skipping Counter 2 Count Source select"]
    #[inline(always)]
    #[must_use]
    pub fn eivtc2(&mut self) -> EIVTC2_W<16> {
        EIVTC2_W::new(self)
    }
    #[doc = "Bits 20:23 - Extended Interrupt Skipping 2 Skipping Count Setting"]
    #[inline(always)]
    #[must_use]
    pub fn eivtt2(&mut self) -> EIVTT2_W<20> {
        EIVTT2_W::new(self)
    }
    #[doc = "Bits 24:27 - Extended Interrupt Skipping Counter 2 Initial Value"]
    #[inline(always)]
    #[must_use]
    pub fn eitcnt2iv(&mut self) -> EITCNT2IV_W<24> {
        EITCNT2IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General PWM Timer Extended Interrupt Skipping Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gteitc](index.html) module"]
pub struct GTEITC_SPEC;
impl crate::RegisterSpec for GTEITC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gteitc::R](R) reader structure"]
impl crate::Readable for GTEITC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gteitc::W](W) writer structure"]
impl crate::Writable for GTEITC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTEITC to value 0"]
impl crate::Resettable for GTEITC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
