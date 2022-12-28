#[doc = "Register `FASTAT` reader"]
pub struct R(crate::R<FASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FASTAT` writer"]
pub struct W(crate::W<FASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FASTAT_SPEC>;
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
impl From<crate::W<FASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFAE` reader - Data Flash Memory Access Violation Flag"]
pub type DFAE_R = crate::BitReader<DFAE_A>;
#[doc = "Data Flash Memory Access Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFAE_A {
    #[doc = "0: No data flash memory access violation has occurred"]
    _0 = 0,
    #[doc = "1: A data flash memory access violation has occurred."]
    _1 = 1,
}
impl From<DFAE_A> for bool {
    #[inline(always)]
    fn from(variant: DFAE_A) -> Self {
        variant as u8 != 0
    }
}
impl DFAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFAE_A {
        match self.bits {
            false => DFAE_A::_0,
            true => DFAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFAE_A::_1
    }
}
#[doc = "Field `DFAE` writer - Data Flash Memory Access Violation Flag"]
pub type DFAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FASTAT_SPEC, DFAE_A, O>;
impl<'a, const O: u8> DFAE_W<'a, O> {
    #[doc = "No data flash memory access violation has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFAE_A::_0)
    }
    #[doc = "A data flash memory access violation has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFAE_A::_1)
    }
}
#[doc = "Field `CMDLK` reader - Command Lock Flag"]
pub type CMDLK_R = crate::BitReader<CMDLK_A>;
#[doc = "Command Lock Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMDLK_A {
    #[doc = "0: The flash sequencer is not in the command-locked state"]
    _0 = 0,
    #[doc = "1: The flash sequencer is in the command-locked state."]
    _1 = 1,
}
impl From<CMDLK_A> for bool {
    #[inline(always)]
    fn from(variant: CMDLK_A) -> Self {
        variant as u8 != 0
    }
}
impl CMDLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDLK_A {
        match self.bits {
            false => CMDLK_A::_0,
            true => CMDLK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMDLK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMDLK_A::_1
    }
}
#[doc = "Field `CFAE` reader - Code Flash Memory Access Violation Flag"]
pub type CFAE_R = crate::BitReader<CFAE_A>;
#[doc = "Code Flash Memory Access Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFAE_A {
    #[doc = "0: No code flash memory access violation has occurred"]
    _0 = 0,
    #[doc = "1: A code flash memory access violation has occurred."]
    _1 = 1,
}
impl From<CFAE_A> for bool {
    #[inline(always)]
    fn from(variant: CFAE_A) -> Self {
        variant as u8 != 0
    }
}
impl CFAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFAE_A {
        match self.bits {
            false => CFAE_A::_0,
            true => CFAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFAE_A::_1
    }
}
#[doc = "Field `CFAE` writer - Code Flash Memory Access Violation Flag"]
pub type CFAE_W<'a, const O: u8> = crate::BitWriter<'a, u8, FASTAT_SPEC, CFAE_A, O>;
impl<'a, const O: u8> CFAE_W<'a, O> {
    #[doc = "No code flash memory access violation has occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFAE_A::_0)
    }
    #[doc = "A code flash memory access violation has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - Data Flash Memory Access Violation Flag"]
    #[inline(always)]
    pub fn dfae(&self) -> DFAE_R {
        DFAE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Lock Flag"]
    #[inline(always)]
    pub fn cmdlk(&self) -> CMDLK_R {
        CMDLK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Code Flash Memory Access Violation Flag"]
    #[inline(always)]
    pub fn cfae(&self) -> CFAE_R {
        CFAE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Data Flash Memory Access Violation Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dfae(&mut self) -> DFAE_W<3> {
        DFAE_W::new(self)
    }
    #[doc = "Bit 7 - Code Flash Memory Access Violation Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfae(&mut self) -> CFAE_W<7> {
        CFAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Access Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fastat](index.html) module"]
pub struct FASTAT_SPEC;
impl crate::RegisterSpec for FASTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fastat::R](R) reader structure"]
impl crate::Readable for FASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fastat::W](W) writer structure"]
impl crate::Writable for FASTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FASTAT to value 0"]
impl crate::Resettable for FASTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
