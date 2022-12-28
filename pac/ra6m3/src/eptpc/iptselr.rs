#[doc = "Register `IPTSELR` reader"]
pub struct R(crate::R<IPTSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTSELR` writer"]
pub struct W(crate::W<IPTSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTSELR_SPEC>;
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
impl From<crate::W<IPTSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPTSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPTSEL0` reader - Pulse Output Timer 0 Select"]
pub type IPTSEL0_R = crate::BitReader<IPTSEL0_A>;
#[doc = "Pulse Output Timer 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL0_A {
    #[doc = "0: Pulse output timer 0 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 0 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL0_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL0_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL0_A {
        match self.bits {
            false => IPTSEL0_A::_0,
            true => IPTSEL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL0_A::_1
    }
}
#[doc = "Field `IPTSEL0` writer - Pulse Output Timer 0 Select"]
pub type IPTSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL0_A, O>;
impl<'a, const O: u8> IPTSEL0_W<'a, O> {
    #[doc = "Pulse output timer 0 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL0_A::_0)
    }
    #[doc = "Pulse output timer 0 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL0_A::_1)
    }
}
#[doc = "Field `IPTSEL1` reader - Pulse Output Timer 1 Select"]
pub type IPTSEL1_R = crate::BitReader<IPTSEL1_A>;
#[doc = "Pulse Output Timer 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL1_A {
    #[doc = "0: Pulse output timer 1 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 1 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL1_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL1_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL1_A {
        match self.bits {
            false => IPTSEL1_A::_0,
            true => IPTSEL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL1_A::_1
    }
}
#[doc = "Field `IPTSEL1` writer - Pulse Output Timer 1 Select"]
pub type IPTSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL1_A, O>;
impl<'a, const O: u8> IPTSEL1_W<'a, O> {
    #[doc = "Pulse output timer 1 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL1_A::_0)
    }
    #[doc = "Pulse output timer 1 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL1_A::_1)
    }
}
#[doc = "Field `IPTSEL2` reader - Pulse Output Timer 2 Select"]
pub type IPTSEL2_R = crate::BitReader<IPTSEL2_A>;
#[doc = "Pulse Output Timer 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL2_A {
    #[doc = "0: Pulse output timer 2 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 2 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL2_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL2_A {
        match self.bits {
            false => IPTSEL2_A::_0,
            true => IPTSEL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL2_A::_1
    }
}
#[doc = "Field `IPTSEL2` writer - Pulse Output Timer 2 Select"]
pub type IPTSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL2_A, O>;
impl<'a, const O: u8> IPTSEL2_W<'a, O> {
    #[doc = "Pulse output timer 2 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL2_A::_0)
    }
    #[doc = "Pulse output timer 2 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL2_A::_1)
    }
}
#[doc = "Field `IPTSEL3` reader - Pulse Output Timer 3 Select"]
pub type IPTSEL3_R = crate::BitReader<IPTSEL3_A>;
#[doc = "Pulse Output Timer 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL3_A {
    #[doc = "0: Pulse output timer 3 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 3 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL3_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL3_A {
        match self.bits {
            false => IPTSEL3_A::_0,
            true => IPTSEL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL3_A::_1
    }
}
#[doc = "Field `IPTSEL3` writer - Pulse Output Timer 3 Select"]
pub type IPTSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL3_A, O>;
impl<'a, const O: u8> IPTSEL3_W<'a, O> {
    #[doc = "Pulse output timer 3 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL3_A::_0)
    }
    #[doc = "Pulse output timer 3 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL3_A::_1)
    }
}
#[doc = "Field `IPTSEL4` reader - Pulse Output Timer 4 Select"]
pub type IPTSEL4_R = crate::BitReader<IPTSEL4_A>;
#[doc = "Pulse Output Timer 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL4_A {
    #[doc = "0: Pulse output timer 4 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 4 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL4_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL4_A {
        match self.bits {
            false => IPTSEL4_A::_0,
            true => IPTSEL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL4_A::_1
    }
}
#[doc = "Field `IPTSEL4` writer - Pulse Output Timer 4 Select"]
pub type IPTSEL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL4_A, O>;
impl<'a, const O: u8> IPTSEL4_W<'a, O> {
    #[doc = "Pulse output timer 4 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL4_A::_0)
    }
    #[doc = "Pulse output timer 4 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL4_A::_1)
    }
}
#[doc = "Field `IPTSEL5` reader - Pulse Output Timer 5 Select"]
pub type IPTSEL5_R = crate::BitReader<IPTSEL5_A>;
#[doc = "Pulse Output Timer 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPTSEL5_A {
    #[doc = "0: Pulse output timer 5 is not selected as a source of IPLS interrupt requests."]
    _0 = 0,
    #[doc = "1: Pulse output timer 5 is selected as a source of IPLS interrupt requests."]
    _1 = 1,
}
impl From<IPTSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: IPTSEL5_A) -> Self {
        variant as u8 != 0
    }
}
impl IPTSEL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPTSEL5_A {
        match self.bits {
            false => IPTSEL5_A::_0,
            true => IPTSEL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPTSEL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPTSEL5_A::_1
    }
}
#[doc = "Field `IPTSEL5` writer - Pulse Output Timer 5 Select"]
pub type IPTSEL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPTSELR_SPEC, IPTSEL5_A, O>;
impl<'a, const O: u8> IPTSEL5_W<'a, O> {
    #[doc = "Pulse output timer 5 is not selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPTSEL5_A::_0)
    }
    #[doc = "Pulse output timer 5 is selected as a source of IPLS interrupt requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPTSEL5_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pulse Output Timer 0 Select"]
    #[inline(always)]
    pub fn iptsel0(&self) -> IPTSEL0_R {
        IPTSEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Select"]
    #[inline(always)]
    pub fn iptsel1(&self) -> IPTSEL1_R {
        IPTSEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Select"]
    #[inline(always)]
    pub fn iptsel2(&self) -> IPTSEL2_R {
        IPTSEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Select"]
    #[inline(always)]
    pub fn iptsel3(&self) -> IPTSEL3_R {
        IPTSEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Select"]
    #[inline(always)]
    pub fn iptsel4(&self) -> IPTSEL4_R {
        IPTSEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Select"]
    #[inline(always)]
    pub fn iptsel5(&self) -> IPTSEL5_R {
        IPTSEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Output Timer 0 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel0(&mut self) -> IPTSEL0_W<0> {
        IPTSEL0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Output Timer 1 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel1(&mut self) -> IPTSEL1_W<1> {
        IPTSEL1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Output Timer 2 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel2(&mut self) -> IPTSEL2_W<2> {
        IPTSEL2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Output Timer 3 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel3(&mut self) -> IPTSEL3_W<3> {
        IPTSEL3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Output Timer 4 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel4(&mut self) -> IPTSEL4_W<4> {
        IPTSEL4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Output Timer 5 Select"]
    #[inline(always)]
    #[must_use]
    pub fn iptsel5(&mut self) -> IPTSEL5_W<5> {
        IPTSEL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPLS Interrupt Request Timer Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptselr](index.html) module"]
pub struct IPTSELR_SPEC;
impl crate::RegisterSpec for IPTSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptselr::R](R) reader structure"]
impl crate::Readable for IPTSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptselr::W](W) writer structure"]
impl crate::Writable for IPTSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPTSELR to value 0"]
impl crate::Resettable for IPTSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
